use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<ApnsFcmOptions>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApnsFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}
