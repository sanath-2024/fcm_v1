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
pub mod aps;
/// OAuth2 authentication helpers.
pub mod auth;
mod client;
/// Platform-independent component of the message.
pub mod message;
pub mod result;
/// Web Push-specific component of the message.
pub mod webpush;
pub use yup_oauth2;

pub use client::Client;
pub use result::{Error, Result};

#[cfg(test)]
mod tests {
    use crate::apns::ApnsConfig;
    use crate::aps::{Alert, Aps, ApsInner};
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn send_simple_noti() {
        let helloworld = "Hello, world!";

        let creds_path = "";
        let project_id = "";
        let registration_token = "".to_string();
        let authenticator = auth::Authenticator::service_account_from_file(creds_path)
            .await
            .unwrap();
        let client = Client::new(authenticator, project_id, false, Duration::from_secs(10));

        let mut test_noti = message::Notification::default();
        test_noti.title = Some("test notification".to_owned());
        test_noti.body = Some(helloworld.to_owned());

        let mut test_message = message::Message::default();
        test_message.token = Some(registration_token);

        test_message.apns = Some(ApnsConfig {
            headers: None,
            payload: Some(Aps {
                aps: Some(ApsInner {
                    mutable_content: Some(1),
                    alert: Some(Alert {
                        title: Some("Test".to_string()),
                        subtitle: Some("Test".to_string()),
                        body: Some("Test".to_string()),
                    }),
                }),
            }),
            fcm_options: None,
        });

        println!("sent:");
        println!("{:#?}", test_message);
        println!("received:");
        println!("{:#?}", client.send(&test_message).await);

        println!(
            "Batch send result: {:#?}",
            client
                .send_batch(&[test_message.clone(), test_message])
                .await
                .unwrap()
        );
    }
}
