pub mod android;
pub mod apns;
mod client;
pub mod message;
mod result;
pub mod webpush;

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
        let client = Client::new(creds_path, project_id, Duration::from_secs(10), false)
            .await
            .unwrap();

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
