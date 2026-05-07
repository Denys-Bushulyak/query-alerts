use std::collections::HashMap;

use regex::{Regex, RegexBuilder};

use crate::entities::{AlertContent, QueryTerm, TermId};

/// Creates a closure that matches [`AlertContent`] against [`QueryTerm`]s
/// using case-insensitive regex.
///
/// Terms are grouped by their language so that only terms written in the
/// same language as an alert content are evaluated against it.
///
/// Regexes are cached keyed by lowercased keyword, so duplicate patterns
/// are built only once.
///
/// # Matching behaviour
///
/// | `keep_order` | Behaviour                                         |
/// |--------------|---------------------------------------------------|
/// | `true`       | The full term text is matched as a single phrase. |
/// | `false`      | Each whitespace-delimited keyword is matched independently; any single match is sufficient. |
///
/// # Returns
///
/// `Some(vec)` containing the [`TermId`] of every matching term (a term may
/// push its id more than once when multiple unordered keywords match), or
/// `None` when nothing matches.
pub fn use_regex_algorithm(
    query_terms: &[QueryTerm],
) -> impl Fn(&AlertContent) -> Option<Vec<TermId>> {
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

        if let Some(terms) = terms_by_language.get(&alert_content.language) {
            for term in terms {
                if term.keep_order {
                    if regexes
                        .get(&term.text.to_lowercase())
                        .expect("Regex should be present!")
                        .is_match(&alert_content.text)
                    {
                        results.push(term.id);
                    }
                } else {
                    let keywords = term.text.split_whitespace();

                    keywords.map(|v| v.to_lowercase()).for_each(|keyword| {
                        if regexes
                            .get(&keyword)
                            .expect("Regex should be present!")
                            .is_match(&alert_content.text)
                        {
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
