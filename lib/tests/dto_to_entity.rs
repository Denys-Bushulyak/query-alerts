use chrono::NaiveDateTime;
use prewave_test_task_lib::{
    dtos::AlertDto,
    entities::{Alert, AlertContent},
};

#[test]
fn convert_alert_dto_to_entity() {
    let alert_dto = AlertDto {
        id: "1".to_string(),
        contents: vec![prewave_test_task_lib::dtos::AlertContentDto {
            text: "@En1Buena Ojalá lo logre eh, y una oferta de Ferrari es inigualable pero la veo fea".to_string(),
            r#type: "text".to_string(),
            language: "es".into(),
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
            language: "es".into(),
        }],
        date: NaiveDateTime::parse_from_str("1996-12-19T16:39:57Z", "%+").unwrap().and_utc(),
        input_type: "tweet".to_string(),
    });
}
