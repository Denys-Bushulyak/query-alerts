use crate::entities::LanguageCode;

/// A localised content entry for an alert (one per language).
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct AlertContent {
    /// The raw text of this content entry.
    pub text: String,
    /// Content type (e.g. `"TITLE"`, `"BODY"`).
    pub r#type: String,
    /// ISO 639-1:2002 language code (e.g. `"en"`, `"fr"`, `"es"`).
    pub language: LanguageCode,
}
