use std::collections::HashMap;

use regex::{Regex, RegexBuilder};

use crate::entities::{AlertContent, QueryTerm, TermId};

/// Returns a closure that matches [`AlertContent`] against a set of [`QueryTerm`]s
/// using case-insensitive regex.
///
/// Two matching modes are supported:
/// - **Ordered** (`keep_order = true`): the entire term text is matched as a single phrase.
/// - **Unordered** (`keep_order = false`): the term text is split on whitespace; matching
///   any individual keyword is sufficient to trigger a match.
///
/// Regexes are cached keyed by the lowercased keyword so duplicate patterns are built
/// only once.  Terms are grouped by language so that content is only compared against
/// terms targeting the same language.
///
/// Returns `Some(ids)` of every matching term (a term may push its id more than once
/// when multiple unordered keywords match), or `None` when nothing matches.
pub fn with_regex(query_terms: &[QueryTerm]) -> impl Fn(&AlertContent) -> Option<Vec<TermId>> {
    let regexes = query_terms.iter().fold(
        HashMap::<String, Regex>::new(),
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

        // Only evaluate terms that target this content's language.
        if let Some(terms) = terms_by_language.get(&alert_content.language) {
            for term in terms {
                if term.keep_order {
                    // Exact phrase match: look for the full term text
                    // in the order specified.
                    let re = regexes
                        .get(&term.text.to_lowercase())
                        .expect("Regex should be present!");
                    if re.is_match(&alert_content.text) {
                        results.push(term.id);
                    }
                } else {
                    // Unordered keyword match: check if any individual
                    // keyword from the term appears in the content.
                    let keywords = term.text.split_whitespace();

                    keywords.map(|v| v.to_lowercase()).for_each(|keyword| {
                        let re = regexes.get(&keyword).expect("Regex should be present!");
                        if re.is_match(&alert_content.text) {
                            results.push(term.id);
                        }
                    })
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
