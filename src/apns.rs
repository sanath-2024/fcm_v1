use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::aps::Aps;

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Aps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<ApnsFcmOptions>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApnsFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
