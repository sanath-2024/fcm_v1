use std::time::Duration;

use serde::Serialize;

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

    /// Send a message. Does not implement retry on failure (that is the caller's responsibility).
    /// Requests the `"https://www.googleapis.com/auth/firebase.messaging"` scope.
    pub async fn send(&self, message: &Message) -> Result<Message> {
        let scopes = &["https://www.googleapis.com/auth/firebase.messaging"];

        let tok = self
            .authenticator
            .token(scopes)
            .await
            .map_err(|_| Error::Auth)?;

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
            .bearer_auth(tok.token().unwrap())
            .json(&req)
            .send()
            .await
            .map_err(|_| Error::Timeout)?;

        if let Err(e) = resp.error_for_status_ref() {
            let http_status = e.status().unwrap();
            return Err(Error::FCM(format!(
                "error code {} ({}): {}",
                http_status.as_u16(),
                http_status.canonical_reason().unwrap(),
                resp.text().await.map_err(|_| Error::Deserialization)?
            )));
        }

        resp.json().await.map_err(|_| Error::Deserialization)
    }
}
