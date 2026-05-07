use std::fs;

use reqwest::Url;

use crate::dtos::QueryTermDto;
use crate::entities::QueryTerm;

/// Fetches query terms from the given API URL and returns a validated list.
///
/// When `debug` is `true`, the raw JSON response is saved to `terms.json`.
pub async fn get_query_terms(entypoint: Url, debug: bool) -> Result<Vec<QueryTerm>, String> {
    let response = reqwest::get(entypoint).await.map_err(|e| e.to_string())?;

    let body = response.text().await.unwrap_or_default();
    if debug {
        fs::write("terms.json", &body).unwrap();
    }

    let dtos: Vec<QueryTermDto> = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    let v = dtos
        .into_iter()
        .map(|dto| dto.try_into())
        .collect::<Result<Vec<QueryTerm>, _>>()?;

    Ok(v)
}
