use chrono::Utc;

use crate::entities::alert_content::AlertContent;

/// Unique identifier for an alert.
pub type AlertId = String;

/// A single alert retrieved from the Prewave API.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Alert {
    /// Unique identifier for this alert.
    pub id: AlertId,
    /// Localised text contents for this alert (one per language).
    pub contents: Vec<AlertContent>,
    /// Timestamp when the alert was published.
    pub date: chrono::DateTime<Utc>,
    /// The original input type (e.g. `"NEWS"`, `"SOCIAL_MEDIA"`).
    pub input_type: String,
}
