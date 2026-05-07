use std::env;

use prewave_test_task_lib::{
    algorithms::with_regex,
    data_providers::{get_alerts::get_alerts, get_query_terms::get_query_terms},
    etc::{get_alert_path, get_query_terms_path},
    query,
};

/// Env var key for the Prewave API base URL.
const API_ENTRYPOINT_ENV_KEY: &str = "API_ENTRYPOINT";
/// Env var key for the Prewave API authentication key.
const API_KEY_ENV_KEY: &str = "API_KEY";
/// Env var key to enable debug mode (set to `"true"` to enable).
pub const DEBUG_MODE_ENV_KEY: &str = "DEBUG";

/// Application entrypoint.
///
/// Reads `API_ENTRYPOINT`, `API_KEY`, and `DEBUG` environment variables,
/// fetches alerts and query terms from the API, runs the matching algorithm,
/// and prints the resulting JSON to stdout.
#[tokio::main]
async fn main() {
    let debug_mode = env::var(DEBUG_MODE_ENV_KEY)
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or_default();

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

    let algo = with_regex(&query_terms);
    let result = query(&alerts, algo);

    match serde_json::to_string(&result) {
        Ok(json) => match debug_mode {
            true => println!("result: {}", json),
            false => println!("{}", json),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
