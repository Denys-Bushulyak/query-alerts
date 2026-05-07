use std::collections::HashMap;

use crate::entities::{AlertContent, QueryTerm, TermId};

/// Creates a closure that matches [`AlertContent`] against [`QueryTerm`]s
/// with a **case-insensitive** keyword strategy.
///
/// Terms are grouped by their language so that only terms written in the
/// same language as an alert content are evaluated against it.
///
/// # Matching behaviour
///
/// | `keep_order` | Behaviour                                                |
/// |--------------|----------------------------------------------------------|
/// | `true`       | Exact substring match — the term's text must appear as a contiguous span in the content. |
/// | `false`      | Unordered keyword match — each whitespace-delimited keyword is lowercased; if *any* keyword appears in the content the term matches. |
///
/// # Returns
///
/// `Some(vec)` containing the [`TermId`] of every matching term, or `None`
/// when no terms match.
pub fn use_case_insensitive_algorithm(
    query_terms: &[QueryTerm],
) -> impl Fn(&AlertContent) -> Option<Vec<TermId>> {
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

    move |alert_content: &AlertContent| {
        let mut results = Vec::new();

        if let Some(terms) = terms_by_language.get(&alert_content.language) {
            for term in terms {
                if term.keep_order {
                    if alert_content.text.contains(&term.text) {
                        results.push(term.id);
                    }
                } else {
                    let keywords = term.text.split_whitespace();

                    if keywords
                        .map(|v| v.to_lowercase())
                        .any(|keyword| alert_content.text.contains(&keyword))
                    {
                        results.push(term.id);
                    };
                }
            }
        }

        if results.len() > 0 {
            Some(results)
        } else {
            None
        }
    }
}
