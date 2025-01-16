use reqwest::Client;

/// Provide a new HTTP client
///
/// # Returns
///
/// * `Client` - A new HTTP client
///
pub fn get_http_client() -> Client {
    Client::new()
}
