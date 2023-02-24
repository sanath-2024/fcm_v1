# basic code generator to help me write out struct definitions for very large objects
# (this is very bad ... the regexes I use for parsing are very fragile)
# I considered using derive_builder, but that doesn't really do what we need.
# Essentially, the request itself is similar to a Rust Builder.

# TODO: investigate properly parsing the discovery document and/or using a procedural macro

import re

message = '''\
pub struct Message {
  "name": string,
  "data": {
    string: string,
    ...
  },
  "notification": {
    object (Notification)
  },
  "android": {
    object (AndroidConfig)
  },
  "webpush": {
    object (WebpushConfig)
  },
  "apns": {
    object (ApnsConfig)
  },
  "fcm_options": {
    object (FcmOptions)
  },

  // Union field target can be only one of the following:
  "token": string,
  "topic": string,
  "condition": string,
  // End of list of possible types for union field target.
}
'''

android_config = '''\
pub struct AndroidConfig {
  "collapse_key": string,
  "priority": enum (AndroidMessagePriority),
  "ttl": string,
  "restricted_package_name": string,
  "data": {
    string: string,
    ...
  },
  "notification": {
    object (AndroidNotification)
  },
  "fcm_options": {
    object (AndroidFcmOptions)
  },
  "direct_boot_ok": boolean,
}'''

android_notification = '''\
pub struct AndroidNotification {
  "title": string,
  "body": string,
  "icon": string,
  "color": string,
  "sound": string,
  "tag": string,
  "click_action": string,
  "body_loc_key": string,
  "body_loc_args": [
    string
  ],
  "title_loc_key": string,
  "title_loc_args": [
    string
  ],
  "channel_id": string,
  "ticker": string,
  "sticky": boolean,
  "event_time": string,
  "local_only": boolean,
  "notification_priority": enum (NotificationPriority),
  "default_sound": boolean,
  "default_vibrate_timings": boolean,
  "default_light_settings": boolean,
  "vibrate_timings": [
    string
  ],
  "visibility": enum (Visibility),
  "notification_count": integer,
  "light_settings": {
    object (LightSettings)
  },
  "image": string,
}'''

webpush_config = '''\
pub struct WebpushConfig {
  "headers": {
    string: string,
    ...
  },
  "data": {
    string: string,
    ...
  },
  "notification": {
    object
  },
  "fcm_options": {
    object (WebpushFcmOptions)
  }
}'''

apns_config = '''\
pub struct ApnsConfig {
  "headers": {
    string: string,
    ...
  },
  "payload": {
    object
  },
  "fcm_options": {
    object (ApnsFcmOptions)
  },
}'''

schemas = [message, android_config, android_notification, webpush_config, apns_config]

comment = (r" *//.*\n", r"")
map = (r"\{\n *string: string,\n *...\n *\}", r"HashMap<String, Value>")
map2 = (r"\{\n *object\n *\}", r"HashMap<String, Value>")
unquote = (r'"([^"]+)"', r"\1")
string_cap = (r"string", r"String")
boolean = (r"boolean", r"bool")
integer = (r"integer", r"i8")
obj = (r"\{\n *object \(([a-zA-Z_]+)\)\n *\}", r"\1")
arr = (r"\[\n *([a-zA-Z_]+)\n *\]", r"Vec<\1>")
enum = (r"enum \(([a-zA-Z_]+)\)", r"\1")
option = (r"( +)([a-zA-Z_]+: )(.+),", r'\1#[serde(skip_serializing_if = "Option::is_none")]\n\1\2Option<\3>,')
pub = (r"( +)([a-zA-Z_]+: )(.+)", r"\1pub \2\3")

patterns = [comment, map, map2, unquote, string_cap, boolean, integer, obj, arr, enum, option, pub]

for schema in schemas:
    for pattern, repl in patterns:
        schema = re.sub(pattern, repl, schema)
    print(schema)
    print()