use std::collections::{HashMap, HashSet};

use crate::entities::{Alert, AlertId, QueryTerm, TermId};

pub fn query(alerts: &[Alert], query_terms: &[QueryTerm]) -> HashMap<TermId, HashSet<AlertId>> {
    // Group terms by language
    let mut terms_by_language: HashMap<&String, Vec<&QueryTerm>> = HashMap::new();
    query_terms.iter().for_each(|term| {
        let terms = terms_by_language.get_mut(&term.language);
        if let Some(terms) = terms {
            terms.push(term);
        } else {
            terms_by_language.insert(&term.language, vec![term]);
        }
    });

    let mut result: HashMap<TermId, HashSet<AlertId>> = HashMap::new();

    alerts.iter().for_each(|alert| {
        alert.contents.iter().for_each(|alert_content| {
            let terms = terms_by_language.get(&alert_content.language);
            if let Some(terms) = terms {
                terms.iter().for_each(|term| {
                    let term_id = term.id;
                    let alert_id = alert.id.clone();

                    if term.keep_order {
                        if alert_content
                            .text
                            .to_lowercase()
                            .contains(&term.text.to_lowercase())
                        {
                            result.entry(term_id).or_default().insert(alert_id.clone());
                        }
                    } else {
                        let term_text = term.text.to_lowercase();
                        let keywords = term_text.split_whitespace();

                        for keyword in keywords {
                            if alert_content.text.to_lowercase().contains(keyword) {
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
