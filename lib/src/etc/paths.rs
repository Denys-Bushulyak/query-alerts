use reqwest::Url;

const ALERTS_PATH: &str = "testAlerts";
const QUERY_TERM_PATH: &str = "testQueryTerm";

/**
 * Constructs the URL for the alert path using the API entrypoint and API key.
 */
pub fn get_alert_path(api_entrypoint: &str, key: &str) -> Result<Url, Box<dyn std::error::Error>> {
    get_path(api_entrypoint, ALERTS_PATH, key)
}

/**
 * Constructs the URL for the query terms path using the API entrypoint and API key.
 */
pub fn get_query_terms_path(
    api_entrypoint: &str,
    api_key: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    get_path(api_entrypoint, QUERY_TERM_PATH, api_key)
}

fn get_path(
    api_entrypoint: &str,
    path: &str,
    key: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    let url = format!("{}/{}?key={}", api_entrypoint, path, key);
    Url::parse(&url).map_err(Into::into)
}
