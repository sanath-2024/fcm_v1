use bytes::Bytes;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::message::Response;
use crate::result::{BatchSendResult, FcmResponseError};
use crate::{auth::Authenticator, message::Message, Error, Result};

/// FCM HTTP v1 API client
#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    authenticator: Authenticator,
    project_id: String,
    validate_only: bool,
    timeout: Duration,
}

#[derive(Serialize)]
struct FCMReq<'a> {
    validate_only: bool,
    message: &'a Message,
}

#[derive(Debug, Deserialize)]
struct FcmError {
    error: FcmErrorInner,
}

#[derive(Debug, Deserialize)]
struct FcmErrorInner {
    details: Vec<FcmErrorDetail>,
}

#[derive(Debug, Deserialize)]
struct FcmErrorDetail {
    #[serde(rename(deserialize = "errorCode"))]
    error_code: Option<String>,
    #[serde(rename(deserialize = "@type"))]
    error_type: String,
}

impl Client {
    /// Build a new client.
    ///
    /// `project_id` is a unique identifier for the project (e.g. myproject-a2bcd).
    ///
    /// `validate_only` is a boolean flag indicating whether the notification should actually be sent out, or if it is just
    /// a test which should only be validated by FCM.
    pub fn new<T: AsRef<str>>(
        authenticator: Authenticator,
        project_id: T,
        validate_only: bool,
        timeout: Duration,
    ) -> Client {
        Client {
            inner: reqwest::Client::new(),
            timeout,
            authenticator,
            project_id: project_id.as_ref().to_owned(),
            validate_only,
        }
    }

    /// Set the timeout for the client. Useful for exponential back-off.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    async fn token(&self) -> Result<String> {
        let scopes = &["https://www.googleapis.com/auth/firebase.messaging"];

        self.authenticator
            .token(scopes)
            .await
            .map_err(|_| Error::Auth)?
            .token()
            .map(|t| t.to_string())
            .ok_or_else(|| Error::Config)
    }

    pub async fn send_batch(&self, messages: &[Message]) -> Result<BatchSendResult> {
        if messages.is_empty() {
            debug_assert!(false);

            return Ok(BatchSendResult {
                all_succeed: true,
                results: vec![],
            })
        }

        if messages.len() == 1 {
            let result = self.send(&messages[0]).await?;

            return Ok(BatchSendResult {
                all_succeed: result.is_none(),
                results: vec![result],
            })
        }

        let mut to_send = "".to_string();

        for message in messages {
            let wrapped = FCMReq {
                validate_only: false,
                message: &message,
            };
            let json = serde_json::to_string(&wrapped).map_err(|_| Error::Deserialization)?;
            let request = format!(
                r#"
--subrequest_boundary
Content-Type: application/http
Content-Transfer-Encoding: binary

POST /v1/projects/{}/messages:send
Content-Type: application/json
accept: application/json

{json}
"#,
                self.project_id
            );

            to_send += &request;
        }

        to_send += "--subrequest_boundary--";

        let token = self.token().await?;
        let mut all_succeed = true;
        let resp = self
            .inner
            .post("https://fcm.googleapis.com/batch")
            .timeout(self.timeout)
            .bearer_auth(token)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("multipart/mixed; boundary=\"subrequest_boundary\""),
            )
            .body(to_send)
            .send()
            .await
            .map_err(|err| Error::FCM(err.to_string()))?;
        let text = resp
            .text()
            .await
            .map_err(|err| Error::FCM(err.to_string()))?;
        let mut result = Vec::with_capacity(messages.len());
        let mut current_json = "".to_string();

        for line in text.lines() {
            if line == "{" {
                debug_assert!(current_json.is_empty());

                current_json += "{";
            } else if line == "}" {
                debug_assert!(!current_json.is_empty());

                current_json += "}";

                let bytes = Bytes::from(current_json);
                let transformed = self.bytes_to_send_result(bytes);

                if transformed.is_some() {
                    all_succeed = false;
                }

                result.push(transformed);

                current_json = "".to_string();
            } else if !current_json.is_empty() {
                current_json += line;
            }
        }

        if result.len() == messages.len() {
            Ok(BatchSendResult {
                all_succeed,
                results: result,
            })
        } else {
            debug_assert!(false, "{text}");

            Err(Error::FCM("Mismatch in return values".to_string()))
        }
    }

    /// Send a message. Does not implement retry on failure (that is the caller's responsibility).
    /// Requests the `"https://www.googleapis.com/auth/firebase.messaging"` scope.
    pub async fn send(&self, message: &Message) -> Result<Option<FcmResponseError>> {
        let tok = self.token().await?;

        let req = FCMReq {
            validate_only: self.validate_only,
            message,
        };

        let resp = self
            .inner
            .post(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                self.project_id
            ))
            .timeout(self.timeout)
            .bearer_auth(tok)
            .json(&req)
            .send()
            .await;

        let resp = resp.map_err(|_| Error::Timeout)?;
        let bytes = resp.bytes().await.map_err(|_| Error::Deserialization)?;

        Ok(self.bytes_to_send_result(bytes))
    }

    fn bytes_to_send_result(&self, bytes: Bytes) -> Option<FcmResponseError> {
        if let Ok(mut error) = serde_json::from_slice::<FcmError>(&bytes) {
            if error.error.details.len() == 1 {
                let detail = error.error.details.remove(0);

                match (&detail.error_code, detail.error_type.as_str()) {
                    (Some(error_code), "type.googleapis.com/google.firebase.fcm.v1.FcmError") => {
                        Some(self.error_message_to_send_result(&error_code))
                    }
                    (_, _) => Some(FcmResponseError::Other(format!(
                        "Got unknown error code, something is very wrong: {:#?}",
                        detail
                    ))),
                }
            } else {
                Some(FcmResponseError::Other(format!(
                    "Detail line is not 1: {}",
                    String::from_utf8_lossy(&bytes).to_string()
                )))
            }
        } else {
            if serde_json::from_slice::<Response>(&bytes).is_ok() {
                None
            } else {
                let string_result = String::from_utf8_lossy(&bytes).to_string();

                Some(FcmResponseError::Other(string_result))
            }
        }
    }

    fn error_message_to_send_result(&self, error_code: &str) -> FcmResponseError {
        match error_code {
            "UNSPECIFIED_ERROR" => FcmResponseError::UnspecifiedError,
            "INVALID_ARGUMENT" => FcmResponseError::InvalidArgument,
            "UNREGISTERED" => FcmResponseError::Unregistered,
            "SENDER_ID_MISMATCH" => FcmResponseError::SenderIdMismatch,
            "QUOTA_EXCEEDED" => FcmResponseError::QuotaExceeded,
            "UNAVAILABLE" => FcmResponseError::Unavailable,
            "INTERNAL" => FcmResponseError::Internal,
            "THIRD_PARTY_AUTH_ERROR" => FcmResponseError::ThirdPartyAuthError,
            _ => FcmResponseError::Other(error_code.to_string()),
        }
    }
}
