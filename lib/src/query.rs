use std::collections::{HashMap, HashSet};

use regex::bytes::RegexBuilder;

use crate::entities::{Alert, AlertId, QueryTerm, TermId};

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
pub fn query(alerts: &[Alert], query_terms: &[QueryTerm]) -> HashMap<TermId, HashSet<AlertId>> {
    // Group terms by language so we only compare content against terms
    // that target the same language.
    let terms_by_language = query_terms.iter().fold(
        HashMap::new(),
        |mut terms_by_language: HashMap<&String, Vec<&QueryTerm>>, term| {
            let terms = terms_by_language.get_mut(&term.language);
            if let Some(terms) = terms {
                terms.push(term);
            } else {
                terms_by_language.insert(&term.language, vec![term]);
            }
            terms_by_language
        },
    );

    // Build a cache of case-insensitive regex patterns for each unique keyword.
    // When `keep_order` is true, the entire term text is treated as one phrase.
    // When false, the text is split into individual whitespace-delimited keywords.
    // Each keyword is stored with its lowercase form as the cache key to avoid
    // building duplicate regexes.
    let regexes = query_terms.iter().fold(
        HashMap::<String, regex::bytes::Regex>::new(),
        |mut regexes, term: &QueryTerm| {
            let keywords = if term.keep_order {
                vec![term.text.as_str()]
            } else {
                term.text.split_whitespace().collect::<Vec<_>>()
            };
            for keyword in keywords {
                if !regexes.contains_key(&keyword.to_lowercase()) {
                    regexes.insert(
                        keyword.to_lowercase().into(),
                        RegexBuilder::new(keyword)
                            .case_insensitive(true)
                            .build()
                            .unwrap(),
                    );
                }
            }
            regexes
        },
    );

    // Iterate over each alert and check its contents against the relevant
    // query terms. A term matches if its regex pattern is found in any content
    // entry that shares the same language code.
    alerts.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<TermId, HashSet<AlertId>>, alert| {
            for content in &alert.contents {
                // Only evaluate terms that target this content's language.
                if let Some(terms) = terms_by_language.get(&content.language) {
                    for term in terms {
                        if term.keep_order {
                            // Exact phrase match: look for the full term text
                            // in the order specified.
                            let re = regexes
                                .get(&term.text.to_lowercase())
                                .expect("Regext should be present!");
                            if re.is_match(&content.text.as_bytes()) {
                                acc.entry(term.id).or_default().insert(alert.id.clone());
                            }
                        } else {
                            // Unordered keyword match: check if any individual
                            // keyword from the term appears in the content.
                            let keywords = term.text.split_whitespace();

                            for keyword in keywords {
                                let re = regexes
                                    .get(&keyword.to_lowercase())
                                    .expect("Regext should be present!");
                                if re.is_match(&content.text.as_bytes()) {
                                    acc.entry(term.id).or_default().insert(alert.id.clone());
                                }
                            }
                        }
                    }
                };
            }

            acc
        },
    )
}
