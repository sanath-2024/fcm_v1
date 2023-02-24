use std::{path::Path, time::Duration};

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use serde::Serialize;
use yup_oauth2::authenticator::Authenticator;

use crate::{message::Message, Error, Result};

pub struct Client {
    inner: reqwest::Client,
    authenticator: Authenticator<HttpsConnector<HttpConnector>>,
    project_id: String,
    validate_only: bool,
}

#[derive(Serialize)]
struct FCMReq<'a> {
    validate_only: bool,
    message: &'a Message,
}

impl Client {
    pub async fn new<P: AsRef<Path>, T: AsRef<str>>(
        service_account_creds_filepath: P,
        project_id: T,
        timeout: Duration,
        validate_only: bool,
    ) -> Result<Client> {
        let creds = yup_oauth2::read_service_account_key(service_account_creds_filepath)
            .await
            .map_err(|_| Error::Auth)?;
        let sa = yup_oauth2::ServiceAccountAuthenticator::builder(creds)
            .build()
            .await
            .map_err(|_| Error::Auth)?;

        Ok(Client {
            inner: reqwest::ClientBuilder::new()
                .timeout(timeout)
                .build()
                .map_err(|_| Error::Config)?,
            authenticator: sa,
            project_id: project_id.as_ref().to_owned(),
            validate_only,
        })
    }

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

        return resp.json().await.map_err(|_| Error::Deserialization);
    }
}
