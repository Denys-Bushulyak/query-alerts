use crate::entities::AlertContent;

#[derive(Debug, serde::Deserialize)]
pub struct AlertContentDto {
    pub text: String,
    pub r#type: String,
    pub language: String,
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
