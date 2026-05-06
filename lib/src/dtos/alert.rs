use chrono::Utc;

use crate::{dtos::alert_content::AlertContentDto, entities::Alert};

#[derive(Debug, serde::Deserialize)]
pub struct AlertDto {
    pub id: String,
    pub contents: Vec<AlertContentDto>,
    pub date: chrono::DateTime<Utc>,
    #[serde(rename(deserialize = "inputType"))]
    pub input_type: String,
}

#[derive(Debug)]
pub enum AlertsError {
    ReqwestError(reqwest::Error),
    ValidationError(String),
}

impl From<reqwest::Error> for AlertsError {
    fn from(error: reqwest::Error) -> Self {
        AlertsError::ReqwestError(error)
    }
}

impl TryFrom<AlertDto> for Alert {
    type Error = AlertsError;

    fn try_from(alert_dto: AlertDto) -> Result<Self, Self::Error> {
        let mut contents = Vec::new();

        for c in alert_dto.contents {
            let content = c.try_into().map_err(|e| {
                AlertsError::ValidationError(format!("Can't parse alert content field: {e:?}"))
            })?;
            contents.push(content);
        }

        Ok(Alert {
            id: alert_dto.id,
            contents,
            date: alert_dto.date,
            input_type: alert_dto.input_type,
        })
    }
}
