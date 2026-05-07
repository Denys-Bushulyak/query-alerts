use std::collections::{HashMap, HashSet};

use fastbloom::BloomFilter;

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

    // Apply bloom filter
    let mut filter = BloomFilter::with_num_bits(2048).expected_items(alerts.len());
    alerts.iter().for_each(|alert| {
        alert.contents.iter().for_each(|c| {
            filter.insert(c.text.as_bytes());
        });
    });

    // Filter alerts by bloom filter
    let alerts = alerts.into_iter().fold(vec![], |mut acc, alert| {
        if alert
            .contents
            .iter()
            .any(|c| !filter.contains(c.text.as_bytes()))
        {
            acc.push(alert);
        }
        acc
    });

    let mut result: HashMap<TermId, HashSet<AlertId>> = HashMap::new();

    alerts.iter().for_each(|alert| {
        alert.contents.iter().for_each(|alert_content| {
            if let Some(terms) = terms_by_language.get(&alert_content.language) {
                terms.iter().for_each(|term| {
                    if term.keep_order {
                        if alert_content
                            .text
                            .to_lowercase()
                            .contains(&term.text.to_lowercase())
                        {
                            let term_id = term.id;
                            let alert_id = alert.id.clone();
                            result.entry(term_id).or_default().insert(alert_id.clone());
                        }
                    } else {
                        let term_text = term.text.to_lowercase();
                        let keywords = term_text.split_whitespace();

                        for keyword in keywords {
                            if alert_content.text.to_lowercase().contains(keyword) {
                                let term_id = term.id;
                                let alert_id = alert.id.clone();
                                result.entry(term_id).or_default().insert(alert_id.clone());
                            }
                        }
                    }
                });
            }
        });
    });

    result
}
