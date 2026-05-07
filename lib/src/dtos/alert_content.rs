use crate::entities::AlertContent;

/// Raw DTO for a localised alert content entry as returned by the API.
#[derive(Debug, serde::Deserialize)]
pub struct AlertContentDto {
    /// The raw text.
    pub text: String,
    /// Content type (e.g. `"TITLE"`, `"BODY"`).
    pub r#type: String,
    /// Language code string (validated non-empty on conversion).
    pub language: String,
}

/// Validation errors for [`AlertContentDto`] fields.
#[derive(Debug)]
pub enum ValidationError {
    /// The `text` field is empty.
    TextIsEmpty,
    /// The `type` field is empty.
    TypeIsEmpty,
    /// The `language` field is empty.
    LanguageIsEmpty,
}

/// Converts an [`AlertContentDto`] into a validated [`AlertContent`] entity.
impl TryFrom<AlertContentDto> for AlertContent {
    type Error = ValidationError;

    fn try_from(alert_content_dto: AlertContentDto) -> Result<Self, Self::Error> {
        if alert_content_dto.text.is_empty() {
            return Err(ValidationError::TextIsEmpty);
        }

        if alert_content_dto.r#type.is_empty() {
            return Err(ValidationError::TypeIsEmpty);
        }

        if alert_content_dto.language.is_empty() {
            return Err(ValidationError::LanguageIsEmpty);
        }

        Ok(AlertContent {
            text: alert_content_dto.text,
            r#type: alert_content_dto.r#type,
            language: alert_content_dto.language,
        })
    }
}
