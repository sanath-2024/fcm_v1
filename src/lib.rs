#![warn(missing_docs)]

//! fcm_v1
//! ======
//!
//! A type-safe way to call the Firebase Cloud Messaging (FCM) HTTP v1 API.
//!
//! OAuth 2.0 authentication is performed via the [yup-oauth2](yup_oauth2) crate.
//! Currently, we request the `"https://www.googleapis.com/auth/firebase.messaging"` scope
//! in order to send messages.

/// Android-specific component of the message.
pub mod android;
/// iOS-specific component of the message.
pub mod apns;
/// OAuth2 authentication helpers.
pub mod auth;
mod client;
/// Platform-independent component of the message.
pub mod message;
mod result;
/// Web Push-specific component of the message.
pub mod webpush;
pub mod aps;

pub use client::Client;
pub use result::{Error, Result};

#[cfg(test)]
mod tests {
    use std::{env, time::Duration};

    use super::*;

    #[tokio::test]
    async fn send_simple_noti() {
        let helloworld = "\u{202e}\u{1d5db}\u{1d5f2}\u{1d5f9}\u{1d5f9}\u{1d5fc}, world!";

        let creds_path = env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap();
        let project_id = env::var("GOOGLE_PROJECT_ID").unwrap();
        let registration_token = env::var("FCM_REGISTRATION_TOKEN").unwrap();
        let authenticator = auth::Authenticator::service_account_from_file(creds_path)
            .await
            .unwrap();
        let client = Client::new(authenticator, project_id, true, Duration::from_secs(10));

        let mut test_noti = message::Notification::default();
        test_noti.title = Some("test notification".to_owned());
        test_noti.body = Some(helloworld.to_owned());

        let mut test_message = message::Message::default();
        test_message.notification = Some(test_noti);
        test_message.token = Some(registration_token);

        println!("sent:");
        println!("{:#?}", test_message);
        println!("received:");
        println!("{:#?}", client.send(&test_message).await.unwrap());
    }
}
