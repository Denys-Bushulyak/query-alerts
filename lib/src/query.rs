use std::collections::{HashMap, HashSet};

use crate::entities::{Alert, AlertContent, AlertId, TermId};

/// Matches query terms against alert contents and returns a mapping of term IDs
/// to sets of matching alert IDs.
///
/// The matching process follows these steps:
/// 1. Groups query terms by their target language.
/// 2. Builds case-insensitive regex patterns for each keyword.
/// 3. Iterates through each alert and its contents.
/// 4. Matches alert content against query terms only when languages align.
/// 5. Returns results as `HashMap<TermId, HashSet<AlertId>>`.
///
/// # Matching Modes
///
/// - **`keep_order: true`** - Treats the term text as an exact phrase. The words
///   must appear in the specified order within the alert content.
/// - **`keep_order: false`** - Splits the term text into individual keywords.
///   A match is found if *any* of the keywords appear in the alert content,
///   regardless of their order.
///
/// # Arguments
///
/// * `alerts` - A slice of [`Alert`] instances to search through. Each alert
///   contains one or more content entries with text and a language code.
/// * `query_terms` - A slice of [`QueryTerm`] instances defining what to search
///   for. Each term includes the search text, target language, and matching mode.
///
/// # Returns
///
/// A [`HashMap`] where each key is a [`TermId`] and the value is a [`HashSet`]
/// of [`AlertId`]s for alerts that matched that term.
///
/// # Panics
///
/// Panics if a regex pattern is expected to exist in the cache but is missing.
/// This should not occur under normal operation since all regexes are built
/// from the provided query terms before matching begins.
pub fn query<Algo>(alerts: &[Alert], algo: Algo) -> HashMap<TermId, HashSet<AlertId>>
where
    Algo: Fn(&AlertContent) -> Option<Vec<TermId>>,
{
    // Iterate over each alert and check its contents against the relevant
    // query terms. A term matches if its regex pattern is found in any content
    // entry that shares the same language code.
    alerts.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<TermId, HashSet<AlertId>>, alert| {
            for content in &alert.contents {
                // Only evaluate terms that target this content's language.
                if let Some(term_ids) = algo(&content) {
                    for term_id in term_ids {
                        acc.entry(term_id).or_default().insert(alert.id.clone());
                    }
                }
            }

            acc
        },
    )
}
