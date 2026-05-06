use std::env;

use prewave_test_task_lib::{
    data_providers::{get_alerts::get_alerts, get_query_terms::get_query_terms},
    etc::{get_alert_path, get_query_terms_path},
    match_alerts,
};

const API_ENTRYPOINT_ENV_KEY: &str = "API_ENTRYPOINT";
const API_KEY_ENV_KEY: &str = "API_KEY";
pub const DEBUG_MODE_ENV_KEY: &str = "DEBUG";

#[tokio::main]
async fn main() {
    let debug_mode = match env::var(DEBUG_MODE_ENV_KEY)
        .unwrap_or_default()
        .parse::<bool>()
    {
        Ok(mode) => mode,
        _ => false,
    };
    println!("Debug mode: {}", debug_mode);

    let entrypoint = env::var(API_ENTRYPOINT_ENV_KEY).expect("API_ENTRYPOINT not set");
    let api_key = env::var(API_KEY_ENV_KEY).expect("API_KEY not set");

    let alerts_entypoint = get_alert_path(&entrypoint, &api_key)
        .inspect(|path| {
            if debug_mode {
                println!("Alert path: {}", path);
            }
        })
        .unwrap();

    let query_terms_entypoint = get_query_terms_path(&entrypoint, &api_key)
        .inspect(|path| {
            if debug_mode {
                println!("Query terms path: {}", path);
            }
        })
        .unwrap();

    let query_terms = get_query_terms(query_terms_entypoint, debug_mode)
        .await
        .unwrap();
    let alerts = get_alerts(alerts_entypoint, debug_mode).await.unwrap();

    let result = match_alerts(&alerts, &query_terms);

    dbg!(result);
}
