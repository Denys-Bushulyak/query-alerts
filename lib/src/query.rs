use std::collections::{HashMap, HashSet};

use regex::bytes::RegexBuilder;

use crate::entities::{Alert, AlertId, QueryTerm, TermId};

pub fn query(alerts: &[Alert], query_terms: &[QueryTerm]) -> HashMap<TermId, HashSet<AlertId>> {
    // Group terms by language
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

    alerts.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<TermId, HashSet<AlertId>>, alert| {
            for content in &alert.contents {
                if let Some(terms) = terms_by_language.get(&content.language) {
                    for term in terms {
                        match term.keep_order {
                            true => {
                                let re = regexes
                                    .get(&term.text.to_lowercase())
                                    .expect("Regext should be present!");
                                if re.is_match(&content.text.as_bytes()) {
                                    acc.entry(term.id).or_default().insert(alert.id.clone());
                                }
                            }
                            false => {
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
                    }
                };
            }

            acc
        },
    )
}
