use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AndroidConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AndroidMessagePriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<AndroidNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<AndroidFcmOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_boot_ok: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AndroidMessagePriority {
    Normal,
    High,
}

impl Default for AndroidMessagePriority {
    fn default() -> AndroidMessagePriority {
        AndroidMessagePriority::Normal
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AndroidNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_loc_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_loc_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_loc_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_loc_args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_priority: Option<NotificationPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_vibrate_timings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_light_settings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vibrate_timings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_count: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_settings: Option<LightSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationPriority {
    PriorityUnspecified,
    PriorityMin,
    PriorityLow,
    PriorityDefault,
    PriorityHigh,
    PriorityMax,
}

impl Default for NotificationPriority {
    fn default() -> NotificationPriority {
        NotificationPriority::PriorityDefault
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visibility {
    VisibilityUnspecified,
    Private,
    Public,
    Secret,
}

impl Default for Visibility {
    fn default() -> Visibility {
        Visibility::Private
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LightSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_on_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_off_duration: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AndroidFcmOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_label: Option<String>,
}
