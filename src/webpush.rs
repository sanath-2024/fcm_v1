use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(missing_docs)]
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

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct WebpushFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}
