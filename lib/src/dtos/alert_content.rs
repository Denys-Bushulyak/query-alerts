use crate::dtos::common::string_to_u8_array;
use crate::entities::{AlertContent, Language};

#[derive(Debug, serde::Deserialize)]
pub struct AlertContentDto {
    pub text: String,
    pub r#type: String,

    /// The language of the alert content as a two-character code (e.g. "en", "fr", "es").
    #[serde(deserialize_with = "string_to_u8_array")]
    pub language: Language,
}

#[derive(Debug)]
pub enum ValidationError {
    TextIsEmpty,
    TypeIsEmpty,
    LanguageIsEmpty,
}

impl TryFrom<AlertContentDto> for AlertContent {
    type Error = ValidationError;

    fn try_from(alert_content_dto: AlertContentDto) -> Result<Self, Self::Error> {
        if alert_content_dto.text.is_empty() {
            return Err(ValidationError::TextIsEmpty);
        }

        if alert_content_dto.r#type.is_empty() {
            return Err(ValidationError::TypeIsEmpty);
        }

        if alert_content_dto.language == Language::default() {
            return Err(ValidationError::LanguageIsEmpty);
        }

        Ok(AlertContent {
            text: alert_content_dto.text,
            r#type: alert_content_dto.r#type,
            language: alert_content_dto.language.into(),
        })
    }
}
