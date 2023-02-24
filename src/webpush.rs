use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WebpushConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<HashMap<String, Value>>,
    pub fcm_options: WebpushFcmOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WebpushFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<String>,
}
