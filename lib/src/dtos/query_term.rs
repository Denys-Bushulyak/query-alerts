use serde::Deserialize;

use crate::entities::QueryTerm;

#[derive(Debug, Deserialize)]
pub struct QueryTermDto {
    pub id: u64,
    pub target: u64,
    pub text: String,
    pub language: String,
    #[serde(rename(deserialize = "keepOrder"))]
    pub keep_order: bool,
}

#[derive(Debug)]
pub enum QueryTermError {
    ReqwestError(reqwest::Error),
    BadResponse(reqwest::StatusCode),
    ValidationError(String),
}

impl TryFrom<QueryTermDto> for QueryTerm {
    type Error = QueryTermError;

    fn try_from(value: QueryTermDto) -> Result<Self, Self::Error> {
        if value.text.trim().is_empty() {
            return Err(QueryTermError::ValidationError(
                "Text field cannot be empty".to_string(),
            ));
        }

        Ok(QueryTerm {
            id: value.id,
            target: value.target,
            text: value.text,
            language: value.language,
            keep_order: value.keep_order,
        })
    }
}
