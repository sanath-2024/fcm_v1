use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{android::AndroidConfig, apns::ApnsConfig, webpush::WebpushConfig};

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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Notification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Target {
    Token(String),
    Topic(String),
    Condition(String),
}

impl Default for Target {
    fn default() -> Target {
        Target::Token(String::default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<String>,
}
