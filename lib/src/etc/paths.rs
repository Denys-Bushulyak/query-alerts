use reqwest::Url;

/// URL path segment for the alerts endpoint.
const ALERTS_PATH: &str = "testAlerts";
/// URL path segment for the query terms endpoint.
const QUERY_TERM_PATH: &str = "testQueryTerm";

/// Constructs the full URL for the alerts API endpoint.
///
/// The returned URL has the form `{api_entrypoint}/testAlerts?key={key}`.
pub fn get_alert_path(api_entrypoint: &str, key: &str) -> Result<Url, Box<dyn std::error::Error>> {
    get_path(api_entrypoint, ALERTS_PATH, key)
}

/// Constructs the full URL for the query terms API endpoint.
///
/// The returned URL has the form `{api_entrypoint}/testQueryTerm?key={key}`.
pub fn get_query_terms_path(
    api_entrypoint: &str,
    api_key: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    get_path(api_entrypoint, QUERY_TERM_PATH, api_key)
}

/// Generic URL builder used by the public path helpers.
fn get_path(
    api_entrypoint: &str,
    path: &str,
    key: &str,
) -> Result<Url, Box<dyn std::error::Error>> {
    let url = format!("{}/{}?key={}", api_entrypoint, path, key);
    Url::parse(&url).map_err(Into::into)
}
