use std::collections::{HashMap, HashSet};

use fastbloom::BloomFilter;
use regex::bytes::RegexBuilder;

use crate::entities::{Alert, AlertId, QueryTerm, TermId};

pub fn query(alerts: &[Alert], query_terms: &[QueryTerm]) -> HashMap<TermId, HashSet<AlertId>> {
    let regexes = query_terms.iter().fold(
        HashMap::<String, regex::bytes::Regex>::new(),
        |mut regexes, term: &QueryTerm| {
            let keywords = if term.keep_order {
                vec![term.text.as_str()]
            } else {
                term.text.split_whitespace().collect::<Vec<_>>()
            };
            for keyword in keywords {
                if !regexes.contains_key(keyword) {
                    regexes.insert(
                        keyword.into(),
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

    // Filter alerts by bloom filter
    let mut filter = BloomFilter::with_num_bits(1024).expected_items(alerts.len());
    alerts.iter().for_each(|alert| {
        alert.contents.iter().for_each(|c| {
            filter.insert(c.text.as_bytes());
        });
    });

    alerts
        .into_iter()
        .flat_map(|alert| {
            alert
                .contents
                .iter()
                .map(|content| (alert.id.clone(), content))
        })
        .filter(|(_, content)| filter.contains(content.text.as_bytes()))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<TermId, HashSet<AlertId>>, (alert_id, alert_content)| {
                if let Some(terms) = terms_by_language.get(&alert_content.language) {
                    terms.iter().for_each(|term| match term.keep_order {
                        true => {
                            let re = regexes.get(&term.text).expect("Regext should be present!");
                            if re.is_match(&alert_content.text.as_bytes()) {
                                acc.entry(term.id).or_default().insert(alert_id.clone());
                            }
                        }
                        false => {
                            let keywords = term.text.split_whitespace();

                            for keyword in keywords {
                                let re = regexes.get(keyword).expect("Regext should be present!");
                                if re.is_match(alert_content.text.as_bytes()) {
                                    acc.entry(term.id).or_default().insert(alert_id.clone());
                                }
                            }
                        }
                    });
                }
                acc
            },
        )
}
