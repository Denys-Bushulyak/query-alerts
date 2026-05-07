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

impl TryFrom<QueryTermDto> for QueryTerm {
    type Error = String;

    fn try_from(value: QueryTermDto) -> Result<Self, Self::Error> {
        if value.text.trim().is_empty() {
            return Err("Text field cannot be empty".to_string());
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
