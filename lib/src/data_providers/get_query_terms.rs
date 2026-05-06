use std::fs;

use reqwest::Url;

use crate::dtos::{QueryTermDto, QueryTermError};
use crate::entities::QueryTerm;

pub async fn get_query_terms(
    entypoint: Url,
    debug: bool,
) -> Result<Vec<QueryTerm>, QueryTermError> {
    let response = reqwest::get(entypoint)
        .await
        .map_err(QueryTermError::ReqwestError)?;

    let body = response.text().await.unwrap_or_default();
    if debug {
        fs::write("terms.json", &body).unwrap();
    }

    let dtos: Vec<QueryTermDto> =
        serde_json::from_str(&body).map_err(|e| QueryTermError::ValidationError(e.to_string()))?;

    let v = dtos
        .into_iter()
        .map(|dto| dto.try_into())
        .collect::<Result<Vec<QueryTerm>, _>>()?;

    Ok(v)
}
