use crate::entities::LanguageCode;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct AlertContent {
    pub text: String,
    pub r#type: String,

    /// I predict that it's an ISO 639-1:2002 language code (e.g. "en", "fr", "es")
    pub language: LanguageCode,
}
