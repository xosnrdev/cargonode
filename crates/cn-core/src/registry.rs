//! npm registry functionality
//!
//! This module provides utilities for interacting with the npm registry,
//! including package name availability checking.

use std::time::Duration;

use reqwest::Client as ReqwestClient;
use serde::Deserialize;

use crate::{Error, Result};

const NPM_REGISTRY_API: &str = "https://registry.npmjs.org";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Response from the npm registry for a package
#[derive(Debug, Deserialize)]
pub struct RegistryResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: std::collections::HashMap<String, String>,
    pub versions: Vec<String>,
}

/// Client for interacting with the npm registry
#[derive(Debug, Clone)]
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    /// Creates a new registry client
    pub fn new() -> Result<Self> {
        let client = ReqwestClient::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|e| Error::Network(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client })
    }

    /// Gets detailed information about a package from the registry
    pub async fn get_package_info(&self, name: &str) -> Result<RegistryResponse> {
        let url = format!("{}/{}", NPM_REGISTRY_API, name);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Network(format!("Registry request failed: {}", e)))?;

        match response.status() {
            reqwest::StatusCode::OK => response
                .json::<RegistryResponse>()
                .await
                .map_err(|e| Error::Registry(format!("Failed to parse registry response: {}", e))),
            reqwest::StatusCode::NOT_FOUND => {
                Err(Error::Registry(format!("Package '{}' not found", name)))
            }
            status => Err(Error::Registry(format!(
                "Unexpected registry response: {} - {}",
                status.as_u16(),
                status.as_str()
            ))),
        }
    }

    /// Checks if a package name is available on the npm registry
    pub async fn is_name_available(&self, name: &str) -> Result<bool> {
        let url = format!("{}/{}", NPM_REGISTRY_API, name);

        match self.client.get(&url).send().await {
            Ok(response) => {
                match response.status() {
                    reqwest::StatusCode::NOT_FOUND => Ok(true), // Name is available
                    reqwest::StatusCode::OK => Ok(false),       // Name is taken
                    status => Err(Error::Registry(format!(
                        "Unexpected registry response: {} - {}",
                        status.as_u16(),
                        status.as_str()
                    ))),
                }
            }
            Err(e) if e.is_timeout() => Err(Error::Network("Registry request timed out".into())),
            Err(e) if e.is_connect() => Err(Error::Network("Failed to connect to registry".into())),
            Err(e) => Err(Error::Network(format!("Registry request failed: {}", e))),
        }
    }
}

/// Checks if a package name is available on the npm registry
///
/// # Arguments
/// * `name` - The package name to check
///
/// # Returns
/// Ok(true) if the name is available, Ok(false) if taken,
/// or Error if the check failed
pub async fn is_name_available(name: &str) -> Result<bool> {
    let client = ReqwestClient::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| Error::Network(format!("Failed to create HTTP client: {}", e)))?;

    let url = format!("{}/{}", NPM_REGISTRY_API, name);

    match client.get(&url).send().await {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::NOT_FOUND => Ok(true), // Name is available
                reqwest::StatusCode::OK => Ok(false),       // Name is taken
                status => Err(Error::Registry(format!(
                    "Unexpected registry response: {} - {}",
                    status.as_u16(),
                    status.as_str()
                ))),
            }
        }
        Err(e) if e.is_timeout() => Err(Error::Network("Registry request timed out".into())),
        Err(e) if e.is_connect() => Err(Error::Network("Failed to connect to registry".into())),
        Err(e) => Err(Error::Network(format!("Registry request failed: {}", e))),
    }
}

/// Gets detailed information about a package from the registry
///
/// # Arguments
/// * `name` - The package name to look up
///
/// # Returns
/// Package information if found, Error if not found or lookup failed
pub async fn get_package_info(name: &str) -> Result<RegistryResponse> {
    let client = ReqwestClient::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| Error::Network(format!("Failed to create HTTP client: {}", e)))?;

    let url = format!("{}/{}", NPM_REGISTRY_API, name);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::Network(format!("Registry request failed: {}", e)))?;

    match response.status() {
        reqwest::StatusCode::OK => response
            .json::<RegistryResponse>()
            .await
            .map_err(|e| Error::Registry(format!("Failed to parse registry response: {}", e))),
        reqwest::StatusCode::NOT_FOUND => {
            Err(Error::Registry(format!("Package '{}' not found", name)))
        }
        status => Err(Error::Registry(format!(
            "Unexpected registry response: {} - {}",
            status.as_u16(),
            status.as_str()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    async fn setup_mock_server() -> (MockServer, String) {
        let server = MockServer::start().await;
        let base_url = server.uri();
        (server, base_url)
    }

    #[tokio::test]
    async fn test_name_available() {
        let (mock_server, base_url) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/test-package"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/test-package", base_url))
            .send()
            .await
            .unwrap();
        assert_eq!(result.status(), reqwest::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_name_taken() {
        let (mock_server, base_url) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/existing-package"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "_id": "existing-package",
                "name": "existing-package",
                "dist-tags": {},
                "versions": ["1.0.0"]
            })))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/existing-package", base_url))
            .send()
            .await
            .unwrap();
        assert_eq!(result.status(), reqwest::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_registry_error() {
        let (mock_server, base_url) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/error-package"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/error-package", base_url))
            .send()
            .await
            .unwrap();
        assert_eq!(result.status(), reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_package_info() {
        let (mock_server, base_url) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/test-package"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "_id": "test-package",
                "name": "test-package",
                "dist-tags": {
                    "latest": "1.0.0"
                },
                "versions": ["1.0.0", "1.1.0", "2.0.0"]
            })))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let response = client
            .get(format!("{}/test-package", base_url))
            .send()
            .await
            .unwrap()
            .json::<RegistryResponse>()
            .await
            .unwrap();

        assert_eq!(response.name, "test-package");
        assert_eq!(response.versions, vec!["1.0.0", "1.1.0", "2.0.0"]);
    }

    #[tokio::test]
    async fn test_registry_timeout() {
        let (mock_server, base_url) = setup_mock_server().await;

        // Configure mock to delay response beyond timeout
        Mock::given(method("GET"))
            .and(path("/slow-package"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(11)))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();

        let result = client
            .get(format!("{}/slow-package", base_url))
            .send()
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().is_timeout());
    }

    #[tokio::test]
    async fn test_registry_malformed_response() {
        let (mock_server, base_url) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/malformed-package"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/malformed-package", base_url))
            .send()
            .await
            .unwrap()
            .json::<RegistryResponse>()
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registry_rate_limit() {
        let (mock_server, base_url) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/rate-limited"))
            .respond_with(ResponseTemplate::new(429).set_body_string("Rate limit exceeded"))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/rate-limited", base_url))
            .send()
            .await
            .unwrap();

        assert_eq!(result.status(), reqwest::StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn test_registry_connection_refused() {
        // Try to connect to a non-existent server
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(1))
            .build()
            .unwrap();

        let result = client
            .get("http://localhost:1") // Use an invalid port
            .send()
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().is_connect());
    }

    #[tokio::test]
    async fn test_registry_redirect() {
        let (mock_server, base_url) = setup_mock_server().await;

        // Set up redirect chain
        Mock::given(method("GET"))
            .and(path("/redirect-package"))
            .respond_with(ResponseTemplate::new(302).insert_header("Location", "/final-package"))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/final-package"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "_id": "redirect-package",
                "name": "redirect-package",
                "dist-tags": {},
                "versions": ["1.0.0"]
            })))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/redirect-package", base_url))
            .send()
            .await
            .unwrap();

        assert_eq!(result.status(), reqwest::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_registry_partial_response() {
        let (mock_server, base_url) = setup_mock_server().await;

        // Return a partial response missing some required fields
        Mock::given(method("GET"))
            .and(path("/partial-package"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "_id": "partial-package",
                // Missing "name" field
                "dist-tags": {},
                "versions": ["1.0.0"]
            })))
            .mount(&mock_server)
            .await;

        let client = ReqwestClient::builder().build().unwrap();

        let result = client
            .get(format!("{}/partial-package", base_url))
            .send()
            .await
            .unwrap()
            .json::<RegistryResponse>()
            .await;

        assert!(result.is_err());
    }
}
