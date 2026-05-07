use chrono::NaiveDateTime;
use prewave_test_task_lib::{
    dtos::{AlertDto, QueryTermDto},
    entities::{Alert, AlertContent, QueryTerm},
};

#[test]
fn convert_alert_dto_to_entity() {
    let alert_dto = AlertDto {
        id: "1".to_string(),
        contents: vec![prewave_test_task_lib::dtos::AlertContentDto {
            text: "@En1Buena Ojalá lo logre eh, y una oferta de Ferrari es inigualable pero la veo fea".to_string(),
            r#type: "text".to_string(),
            language: "ês".into(),
        }],
        date: NaiveDateTime::parse_from_str("1996-12-19T16:39:57Z", "%+").unwrap().and_utc(),
        input_type: "tweet".to_string(),
    };
    let alert_entity: Alert = alert_dto.try_into().unwrap();

    assert_eq!(alert_entity, Alert{
        id: "1".to_string(),
        contents: vec![AlertContent {
            text: "@En1Buena Ojalá lo logre eh, y una oferta de Ferrari es inigualable pero la veo fea".to_string(),
            r#type: "text".to_string(),
            language: "ês".into(),
        }],
        date: NaiveDateTime::parse_from_str("1996-12-19T16:39:57Z", "%+").unwrap().and_utc(),
        input_type: "tweet".to_string(),
    });
}

#[test]
fn convert_term_dto_to_entity() {
    let query_term_dto = QueryTermDto {
        id: 1,
        target: 5,
        text: "hello world".to_string(),
        language: "en".to_string(),
        keep_order: true,
    };
    let expected_query_term = QueryTerm {
        id: 1,
        target: 5,
        text: "hello world".to_string(),
        language: "en".to_string(),
        keep_order: true,
    };
    let query_term_entity: QueryTerm = query_term_dto.try_into().unwrap();

    assert_eq!(query_term_entity, expected_query_term);

    // Test case for empty text
    let invalid_dto = QueryTermDto {
        id: 2,
        target: 6,
        text: "  ".to_string(),
        language: "fr".to_string(),
        keep_order: false,
    };
    assert!(QueryTerm::try_from(invalid_dto).is_err());
}
