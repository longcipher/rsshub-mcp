//! RSSHub API Client Library
//!
//! This crate provides a Rust client for interacting with RSSHub APIs,
//! allowing you to fetch namespace information, radar rules, and category data.

#![allow(unused)]
use std::{collections::HashMap, time::Duration};

use eyre::Result;
use serde::{Deserialize, Serialize};

const DEFAULT_HOST: &str = "https://rsshub.akjong.com";
const DEFAULT_TIMEOUT: u64 = 120;

#[derive(Debug, Clone, Default)]
pub struct RsshubClientConfig {
    pub host: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Default, Debug, Clone)]
pub struct RsshubApiClient {
    pub client: reqwest::Client,
    pub host: String,
}

impl RsshubApiClient {
    pub fn new(config: RsshubClientConfig) -> Self {
        // Use default values if not provided in config
        let host = config.host.as_deref().unwrap_or(DEFAULT_HOST);
        let timeout = config.timeout.unwrap_or(DEFAULT_TIMEOUT);
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .expect("Failed to build HTTP client"),
            host: host.to_string(),
        }
    }

    pub async fn get_all_namespaces(&self) -> Result<NamespaceResp> {
        let url = format!("{}/api/namespace", self.host);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            let routes: NamespaceResp = response.json().await?;
            Ok(routes)
        } else {
            Err(eyre::eyre!("Failed to fetch namespaces"))
        }
    }

    pub async fn get_namespace(&self, namespace: &str) -> Result<RoutesMap> {
        let url = format!("{}/api/namespace/{}", self.host, namespace);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            let route: RoutesMap = response.json().await?;
            Ok(route)
        } else {
            Err(eyre::eyre!("Failed to fetch namespace"))
        }
    }

    pub async fn get_all_radar_rules(&self) -> Result<RulesResp> {
        let url = format!("{}/api/radar/rules", self.host);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            let rules: RulesResp = response.json().await?;
            Ok(rules)
        } else {
            Err(eyre::eyre!("Failed to fetch radar rules"))
        }
    }

    pub async fn get_radar_rule(&self, domain: &str) -> Result<RulesInfo> {
        let url = format!("{}/api/radar/rules/{}", self.host, domain);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            let rule: RulesInfo = response.json().await?;
            Ok(rule)
        } else {
            Err(eyre::eyre!("Failed to fetch radar rule"))
        }
    }

    pub async fn get_category(&self, category: &str) -> Result<CategoryItems> {
        let url = format!("{}/api/category/{}", self.host, category);
        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            let category: CategoryItems = response.json().await?;
            Ok(category)
        } else {
            Err(eyre::eyre!("Failed to fetch category"))
        }
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

// 代表每个域名下的信息，例如 "81.cn" 对应的值
#[derive(Deserialize, Serialize, Debug, Clone)] // 添加 Clone 以便需要时复制
pub struct RulesInfo {
    // 使用 serde 重命名 _name 字段，因为 Rust 标识符通常不以下划线开头
    #[serde(rename = "_name")]
    pub name: String,

    // 使用 #[serde(flatten)] 来捕获所有其他的键 (像 "81rc", "ds", ".", "www" 等)
    // 这些键的值是一个 RouteInfo 对象的数组 (Vec<RouteInfo>)
    #[serde(flatten)]
    pub sections: HashMap<String, Vec<RouteInfo>>,
}

// 代表每个路由规则的具体信息，位于数组中
#[derive(Deserialize, Serialize, Debug, Clone)] // 添加 Clone
pub struct RouteInfo {
    pub title: String,
    pub docs: String,
    pub source: Vec<String>, // source 是一个字符串数组
    pub target: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryItems(pub HashMap<String, CategoryInfo>); // Top-level map

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
}
