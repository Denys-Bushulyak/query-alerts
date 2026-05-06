use crate::entities::LanguageCode;

pub type TermId = u64;

#[derive(Debug, Eq, PartialEq)]
pub struct QueryTerm {
    pub id: TermId,
    pub target: u64,
    pub text: String,
    /// I predict that it's an ISO 639-1:2002 language code (e.g. "en", "fr", "es")
    pub language: LanguageCode,
    pub keep_order: bool,
}
