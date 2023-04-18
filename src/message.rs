use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{android::AndroidConfig, apns::ApnsConfig, webpush::WebpushConfig};

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<AndroidConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webpush: Option<WebpushConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<ApnsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<FcmOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

/// The target of the push notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Target {
    /// Registration token to send a message to. Use this option to send notifications to individual devices.
    Token(String),
    /// Topic name to send a message to, e.g. `"weather"`. Note: `"/topics/"` prefix should not be provided.
    Topic(String),
    /// Condition to send a message to, e.g. `"'foo' in topics && 'bar' in topics"`.
    Condition(String),
}

impl Default for Target {
    fn default() -> Target {
        Target::Token(String::default())
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}
