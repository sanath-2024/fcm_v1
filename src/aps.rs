use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Aps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aps: Option<ApsInner>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ApsInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "mutable-content"))]
    // Either 0 or 1, not sure if it's possible that serde can map bool to i32 itself
    pub mutable_content: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<Alert>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
