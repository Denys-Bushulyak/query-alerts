use std::fs;

use reqwest::Url;

use crate::{
    dtos::{AlertDto, AlertsError},
    entities::Alert,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum GetAlertsError {
    HttpError(reqwest::Error),
    JsonError(serde_json::Error),
    ValidationError(AlertsError),
}

pub async fn get_alerts(entrypoint: Url, debug: bool) -> Result<Vec<Alert>, GetAlertsError> {
    let response = reqwest::get(entrypoint)
        .await
        .map_err(GetAlertsError::HttpError)?;

    let body = response.text().await.unwrap_or_default();
    if debug {
        fs::write("alerts.json", &body).unwrap();
    }

    let dtos: Vec<AlertDto> =
        serde_json::from_str(&body).map_err(GetAlertsError::JsonError)?;

    dtos.into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Alert>, _>>()
        .map_err(GetAlertsError::ValidationError)
}
