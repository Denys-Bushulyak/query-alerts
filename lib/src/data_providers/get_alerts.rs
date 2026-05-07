use std::fs;

use reqwest::Url;

use crate::{dtos::AlertDto, entities::Alert};

/// Fetches alerts from the given API URL and returns a validated list.
///
/// When `debug` is `true`, the raw JSON response is saved to `alerts.json`.
pub async fn get_alerts(entrypoint: Url, debug: bool) -> Result<Vec<Alert>, String> {
    let response = reqwest::get(entrypoint).await.map_err(|e| e.to_string())?;

    let body = response.text().await.unwrap_or_default();
    if debug {
        fs::write("alerts.json", &body).unwrap();
    }

    let dtos: Vec<AlertDto> = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    dtos.into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<Alert>, _>>()
        .map_err(|e| e.to_string())
}
