use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub r#type: NotificationType,
    pub title: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationType {
    Err,
    Warn,
    Info,
}
