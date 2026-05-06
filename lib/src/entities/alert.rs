use chrono::Utc;

use crate::entities::alert_content::AlertContent;

pub type AlertId = String;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Alert {
    pub id: AlertId,
    pub contents: Vec<AlertContent>,
    pub date: chrono::DateTime<Utc>,
    pub input_type: String,
}
