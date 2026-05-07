use chrono::Utc;

use crate::{dtos::alert_content::AlertContentDto, entities::Alert};

/// Raw DTO for an alert as returned by the API.
#[derive(Debug, serde::Deserialize)]
pub struct AlertDto {
    /// Unique alert identifier.
    pub id: String,
    /// Localised content entries.
    pub contents: Vec<AlertContentDto>,
    /// Publication timestamp.
    pub date: chrono::DateTime<Utc>,
    /// Input type (deserialised from `"inputType"` in JSON).
    #[serde(rename(deserialize = "inputType"))]
    pub input_type: String,
}

/// Converts an [`AlertDto`] into a validated [`Alert`] entity.
impl TryFrom<AlertDto> for Alert {
    type Error = String;

    fn try_from(alert_dto: AlertDto) -> Result<Self, Self::Error> {
        let mut contents = Vec::new();

        for c in alert_dto.contents {
            let content = c
                .try_into()
                .map_err(|e| format!("Can't parse alert content field: {e:?}"))?;
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
