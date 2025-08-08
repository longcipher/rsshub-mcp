//! RSSHub API Client Library
//!
//! This crate provides a Rust client for interacting with RSSHub APIs,
//! allowing you to fetch namespace information, radar rules, and category data.

#![allow(unused)]
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use eyre::Result;
use serde::{Deserialize, Serialize};

const DEFAULT_HOST: &str = "https://rsshub.akjong.com";
const DEFAULT_TIMEOUT: u64 = 120;

#[derive(Debug, Clone, Default)]
pub struct RsshubClientConfig {
    pub host: Option<String>,
    pub timeout: Option<u64>,
    pub retries: Option<u32>,
    pub retry_backoff_ms: Option<u64>,
    pub namespaces_ttl_secs: Option<u64>,
    pub radar_rules_ttl_secs: Option<u64>,
}

#[derive(Default, Debug, Clone)]
pub struct RsshubApiClient {
    pub client: reqwest::Client,
    pub host: String,
    cache: Arc<std::sync::Mutex<CacheStore>>,
    retries: u32,
    retry_backoff_ms: u64,
    namespaces_ttl_secs: u64,
    radar_rules_ttl_secs: u64,
}

impl RsshubApiClient {
    pub fn new(config: RsshubClientConfig) -> Self {
        // Use default values if not provided in config
        let host = config.host.as_deref().unwrap_or(DEFAULT_HOST);
        let timeout = config.timeout.unwrap_or(DEFAULT_TIMEOUT);
        let retries = config.retries.unwrap_or(3);
        let retry_backoff_ms = config.retry_backoff_ms.unwrap_or(150);
        let namespaces_ttl_secs = config.namespaces_ttl_secs.unwrap_or(300);
        let radar_rules_ttl_secs = config.radar_rules_ttl_secs.unwrap_or(600);
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .expect("Failed to build HTTP client"),
            host: host.to_string(),
            cache: Arc::new(std::sync::Mutex::new(CacheStore::default())),
            retries,
            retry_backoff_ms,
            namespaces_ttl_secs,
            radar_rules_ttl_secs,
        }
    }

    async fn get_with_retry(&self, url: &str) -> Result<reqwest::Response> {
        let mut last_err = None;
        for _ in 0..self.retries {
            match self.client.get(url).send().await {
                Ok(resp) => return Ok(resp),
                Err(e) => {
                    last_err = Some(e);
                    tokio::time::sleep(Duration::from_millis(self.retry_backoff_ms)).await;
                }
            }
        }
        Err(eyre::eyre!(
            "HTTP GET failed after retries: {}",
            last_err.map(|e| e.to_string()).unwrap_or_default()
        ))
    }

    pub async fn get_all_namespaces(&self) -> Result<NamespaceResp> {
        let url = format!("{}/api/namespace", self.host);
        // Cache using configured TTL
        if let Some(v) = self
            .cache
            .lock()
            .expect("Failed to lock cache mutex")
            .get_json("namespaces", self.namespaces_ttl_secs)
        {
            return Ok(serde_json::from_value(v)?);
        }
        let response = self.get_with_retry(&url).await?;
        if response.status().is_success() {
            let routes: NamespaceResp = response.json().await?;
            self.cache
                .lock()
                .expect("Failed to lock cache mutex")
                .put_json("namespaces", &serde_json::to_value(&routes)?);
            Ok(routes)
        } else {
            Err(eyre::eyre!("Failed to fetch namespaces"))
        }
    }

    pub async fn get_namespace(&self, namespace: &str) -> Result<RoutesMap> {
        let url = format!("{}/api/namespace/{}", self.host, namespace);
        let response = self.get_with_retry(&url).await?;
        if response.status().is_success() {
            let route: RoutesMap = response.json().await?;
            Ok(route)
        } else {
            Err(eyre::eyre!("Failed to fetch namespace"))
        }
    }

    pub async fn get_all_radar_rules(&self) -> Result<RulesResp> {
        let url = format!("{}/api/radar/rules", self.host);
        // Cache using configured TTL
        if let Some(v) = self
            .cache
            .lock()
            .expect("Failed to lock cache mutex")
            .get_json("radar_rules", self.radar_rules_ttl_secs)
        {
            return Ok(serde_json::from_value(v)?);
        }
        let response = self.get_with_retry(&url).await?;
        if response.status().is_success() {
            let rules: RulesResp = response.json().await?;
            self.cache
                .lock()
                .expect("Failed to lock cache mutex")
                .put_json("radar_rules", &serde_json::to_value(&rules)?);
            Ok(rules)
        } else {
            Err(eyre::eyre!("Failed to fetch radar rules"))
        }
    }

    pub async fn get_radar_rule(&self, domain: &str) -> Result<RulesInfo> {
        let url = format!("{}/api/radar/rules/{}", self.host, domain);
        let response = self.get_with_retry(&url).await?;
        if response.status().is_success() {
            let rule: RulesInfo = response.json().await?;
            Ok(rule)
        } else {
            Err(eyre::eyre!("Failed to fetch radar rule"))
        }
    }

    pub async fn get_category(&self, category: &str) -> Result<CategoryItems> {
        let url = format!("{}/api/category/{}", self.host, category);
        let response = self.get_with_retry(&url).await?;
        if response.status().is_success() {
            let category: CategoryItems = response.json().await?;
            Ok(category)
        } else {
            Err(eyre::eyre!("Failed to fetch category"))
        }
    }

    /// Fetch RSS feed content from a RSSHub route
    pub async fn get_feed(&self, path: &str) -> Result<FeedResponse> {
        let path = path.strip_prefix('/').unwrap_or(path);
        let url = format!("{}/{}", self.host, path);
        let response = self.get_with_retry(&url).await?;
        if response.status().is_success() {
            let content = response.text().await?;
            let feed = self.parse_rss_content(&content)?;
            Ok(feed)
        } else {
            Err(eyre::eyre!("Failed to fetch RSS feed from path: {}", path))
        }
    }

    /// Parse RSS content using feedparser-like logic
    fn parse_rss_content(&self, content: &str) -> Result<FeedResponse> {
        // Try RSS first
        if let Ok(channel) = rss::Channel::read_from(content.as_bytes()) {
            let items = channel
                .items()
                .iter()
                .map(|it| FeedItem {
                    title: it.title().unwrap_or("").to_string(),
                    description: it.description().unwrap_or("").to_string(),
                    link: it.link().unwrap_or("").to_string(),
                    pub_date: it.pub_date().map(|s| s.to_string()),
                    author: it.author().map(|s| s.to_string()),
                    categories: it
                        .categories()
                        .iter()
                        .map(|c| c.name().to_string())
                        .collect(),
                })
                .collect();
            return Ok(FeedResponse {
                title: channel.title().to_string(),
                description: channel.description().to_string(),
                items,
                raw_content: Some(content.to_string()),
            });
        }

        // Fallback: return raw as before
        Ok(FeedResponse {
            title: "RSS Feed".to_string(),
            description: "RSS feed content".to_string(),
            items: vec![],
            raw_content: Some(content.to_string()),
        })
    }
}

#[derive(Default, Debug)]
struct CacheStore {
    json: HashMap<String, (serde_json::Value, Instant)>,
}

impl CacheStore {
    fn get_json(&self, key: &str, ttl_secs: u64) -> Option<serde_json::Value> {
        self.json.get(key).and_then(|(v, t)| {
            if t.elapsed().as_secs() <= ttl_secs {
                Some(v.clone())
            } else {
                None
            }
        })
    }
    fn put_json(&mut self, key: &str, v: &serde_json::Value) {
        self.json
            .insert(key.to_string(), (v.clone(), Instant::now()));
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RouteDetails {
    pub path: MultiType,
    pub name: String,
    // Optional because not all routes have a specific 'url' field inside them
    #[serde(default)]
    pub url: Option<String>,
    pub maintainers: Vec<String>,
    pub example: Option<String>,
    // Parameters can be absent or an empty object
    #[serde(default)]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    pub description: Option<String>,
    pub categories: Option<Vec<String>>,
    pub features: Option<Features>,
    // Radar can be absent
    #[serde(default)]
    pub radar: Option<RadarType>,
    pub location: Option<String>,
    pub view: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MultiType {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum RadarType {
    Single(RadarItem),
    Multiple(Vec<RadarItem>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // Matches the JSON keys like requireConfig
pub struct Features {
    pub require_config: Option<RequireConfig>, // Renamed to snake_case in Rust
    pub require_puppeteer: Option<bool>,
    pub anti_crawler: Option<bool>,
    pub support_radar: Option<bool>,
    #[serde(rename = "supportBT")]
    pub support_bt: Option<bool>,
    pub support_podcast: Option<bool>,
    pub support_scihub: Option<bool>,
}

// Represents the "requireConfig" field, which can be a boolean or a list of objects
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)] // Tells serde to try deserializing as one variant, then the next
pub enum RequireConfig {
    Bool(bool),
    List(Vec<ConfigDetail>),
}

// Represents an object within the "requireConfig" list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigDetail {
    pub name: String,
    pub optional: Option<bool>,
    pub description: Option<String>,
}

// Represents an object within the "radar" array
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RadarItem {
    pub source: MultiType,
    // Target is not always present
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RoutesMap {
    pub routes: Option<HashMap<String, RouteDetails>>,
}

pub type NamespaceResp = HashMap<String, RoutesMap>;

pub type RulesResp = HashMap<String, RulesInfo>;

// Represents information under each domain, e.g., the value corresponding to "81.cn"
#[derive(Deserialize, Serialize, Debug, Clone)] // Add Clone for copying when needed
pub struct RulesInfo {
    // Use serde to rename the _name field, as Rust identifiers typically don't start with underscore
    #[serde(rename = "_name")]
    pub name: String,

    // Use #[serde(flatten)] to capture all other keys (like "81rc", "ds", ".", "www", etc.)
    // The values of these keys are arrays of RouteInfo objects (Vec<RouteInfo>)
    #[serde(flatten)]
    pub sections: HashMap<String, Vec<RouteInfo>>,
}

// Represents specific information for each routing rule, located in the array
#[derive(Deserialize, Serialize, Debug, Clone)] // Add Clone
pub struct RouteInfo {
    pub title: String,
    pub docs: String,
    pub source: Vec<String>, // source is a string array
    pub target: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryItems(pub HashMap<String, CategoryInfo>); // Top-level map

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedResponse {
    pub title: String,
    pub description: String,
    pub items: Vec<FeedItem>,
    pub raw_content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedItem {
    pub title: String,
    pub description: String,
    pub link: String,
    pub pub_date: Option<String>,
    pub author: Option<String>,
    pub categories: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")] // Handle potential camelCase in top-level service fields if any
pub struct CategoryInfo {
    pub name: String,
    pub url: Option<String>,
    #[serde(default)] // Use default (empty vec) if missing
    pub categories: Vec<String>,
    pub description: Option<String>, // Seems consistently present for Service itself
    pub lang: Option<String>,        // Seems consistently present for Service itself
    pub routes: HashMap<String, RouteDetails>,
    // Added based on example like "163", might be optional overall
    #[serde(default)]
    pub zh: Option<ZhTranslation>,
}

// Optional nested structure for zh translations if present
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZhTranslation {
    pub name: Option<String>,
    pub description: Option<String>,
    // Add other potentially translated fields if needed
    pub path: Option<String>,
    pub maintainers: Option<Vec<String>>,
    pub example: Option<String>,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[tokio::test]
    async fn test_get_all_namespaces() {
        let mut server = mockito::Server::new_async().await;
        let mock_endpoint = server
            .mock("GET", "/api/namespace")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/namespace.json")
            .create_async()
            .await;

        // Create client with mocked server URL
        let config = RsshubClientConfig {
            host: Some(server.url()),
            timeout: Some(60),
            ..Default::default()
        };
        let client = RsshubApiClient::new(config);

        // Call the API which will hit our mock server
        let result = client.get_all_namespaces().await;

        // Verify the mock was called
        mock_endpoint.assert_async().await;

        println!("result: {:?}", result);

        // Verify result
        assert!(
            result.is_ok(),
            "Failed to fetch namespaces from mock server"
        );
        let routes = result.unwrap();
        assert!(
            !routes.is_empty(),
            "Routes map from mock server should not be empty"
        );
    }

    #[tokio::test]
    async fn test_get_namespace() {
        // Create default config
        let config = RsshubClientConfig::default();
        let client = RsshubApiClient::new(config);
        let result = client.get_namespace("zyw").await;
        assert!(result.is_ok(), "Failed to fetch namespaces");
        // let routes = result.unwrap();
        // println!("{:#?}", routes);
    }

    #[tokio::test]
    async fn test_get_all_radar_rules() {
        let mut server = mockito::Server::new_async().await;
        let mock_endpoint = server
            .mock("GET", "/api/radar/rules")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/rules.json")
            .create_async()
            .await;

        // Create client with mocked server URL
        let config = RsshubClientConfig {
            host: Some(server.url()),
            timeout: Some(60),
            ..Default::default()
        };
        let client = RsshubApiClient::new(config);

        // Call the API which will hit our mock server
        let result = client.get_all_radar_rules().await;

        // Verify the mock was called
        mock_endpoint.assert_async().await;

        println!("result: {:?}", result);

        // Verify result
        assert!(
            result.is_ok(),
            "Failed to fetch radar rules from mock server"
        );
    }

    #[tokio::test]
    async fn test_get_radar_rule() {
        let config = RsshubClientConfig::default();
        let client = RsshubApiClient::new(config);
        // Call the API which will hit our mock server
        let result = client.get_radar_rule("81.cn").await;
        println!("result: {:?}", result);

        // Verify result
        assert!(
            result.is_ok(),
            "Failed to fetch radar rule from mock server"
        );
    }

    #[tokio::test]
    async fn test_get_category() {
        let mut server = mockito::Server::new_async().await;
        let mock_endpoint = server
            .mock("GET", "/api/category/new-media")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/category.json")
            .create_async()
            .await;

        // Create client with mocked server URL
        let config = RsshubClientConfig {
            host: Some(server.url()),
            timeout: Some(60),
            ..Default::default()
        };
        let client = RsshubApiClient::new(config);

        // Call the API which will hit our mock server
        let result = client.get_category("new-media").await;

        // Verify the mock was called
        mock_endpoint.assert_async().await;

        println!("result: {:?}", result);

        // Verify result
        assert!(result.is_ok(), "Failed to fetch category from mock server");
    }

    #[tokio::test]
    async fn test_cache_ttl_behavior() {
        // Spin up mock server for namespaces
        let mut server = mockito::Server::new_async().await;
        let mock_endpoint = server
            .mock("GET", "/api/namespace")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/namespace.json")
            .create_async()
            .await;

        let config = RsshubClientConfig {
            host: Some(server.url()),
            timeout: Some(60),
            retries: Some(1),
            retry_backoff_ms: Some(10),
            namespaces_ttl_secs: Some(1),
            radar_rules_ttl_secs: Some(600),
        };
        let client = RsshubApiClient::new(config);

        // First call hits server and caches
        let _ = client.get_all_namespaces().await.unwrap();
        mock_endpoint.assert_async().await;

        // Second call within TTL should be served from cache (no new HTTP call)
        let _ = client.get_all_namespaces().await.unwrap();

        // Sleep to expire TTL
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Third call after TTL would call server again; create another mock to assert
        let mock_endpoint2 = server
            .mock("GET", "/api/namespace")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/namespace.json")
            .create_async()
            .await;
        let _ = client.get_all_namespaces().await.unwrap();
        mock_endpoint2.assert_async().await;
    }

    #[test]
    fn test_parser_fallback_for_non_rss() {
        let client = RsshubApiClient::new(RsshubClientConfig::default());
        // Provide HTML instead of RSS to trigger fallback
        let html = "<html><head><title>Not RSS</title></head><body>Hello</body></html>";
        let parsed = client.parse_rss_content(html).unwrap();
        assert!(parsed.raw_content.is_some());
        assert!(parsed.items.is_empty());
        assert_eq!(parsed.title, "RSS Feed");
    }
}
