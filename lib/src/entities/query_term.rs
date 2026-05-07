use crate::entities::LanguageCode;

/// Unique identifier for a query term.
pub type TermId = u64;

/// A single query term used to match against alert contents.
#[derive(Debug, Eq, PartialEq)]
pub struct QueryTerm {
    /// Unique identifier for this term.
    pub id: TermId,
    /// Target group or category this term belongs to.
    pub target: u64,
    /// The search text — either a phrase or space-separated keywords.
    pub text: String,
    /// ISO 639-1:2002 language code (e.g. `"en"`, `"fr"`, `"es"`).
    pub language: LanguageCode,
    /// If `true`, match as an exact phrase; otherwise match any keyword.
    pub keep_order: bool,
}
