use std::sync::Arc;

use async_trait::async_trait;
use rsshub_api::{RsshubApiClient, RsshubClientConfig};
use serde_json::json;
use tracing::info;
use ultrafast_mcp::{
    prelude::*,
    types::{ToolCallRequest, ToolCallResponse},
    ListToolsRequest, ListToolsResponse, ToolContent,
};

/// RSSHub MCP Service that implements both ToolHandler and ResourceHandler
#[derive(Debug)]
pub struct RSSHubService {
    client: Arc<RsshubApiClient>,
}

impl RSSHubService {
    /// Create a new RSSHubService with default configuration
    pub fn new() -> Self {
        let config = RsshubClientConfig::default();
        let client = Arc::new(RsshubApiClient::new(config));
        Self { client }
    }

    /// Handle search_routes tool call
    async fn handle_search_routes(
        &self,
        query: &str,
        namespace: Option<&str>,
        limit: Option<usize>,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let q = query.to_lowercase();
        let limit = limit.unwrap_or(20);

        // Helper to check match
        let matches = |key: &str, details: &rsshub_api::RouteDetails| {
            let key_m = key.to_lowercase().contains(&q);
            let name_m = details.name.to_lowercase().contains(&q);
            let desc_m = details
                .description
                .as_ref()
                .map(|d| d.to_lowercase().contains(&q))
                .unwrap_or(false);
            let ex_m = details
                .example
                .as_ref()
                .map(|e| e.to_lowercase().contains(&q))
                .unwrap_or(false);
            key_m || name_m || desc_m || ex_m
        };

        let mut hits: Vec<serde_json::Value> = Vec::new();

        if let Some(ns) = namespace {
            let routes_map = self.client.get_namespace(ns).await?;
            if let Some(routes) = routes_map.routes {
                for (key, details) in routes.iter() {
                    if matches(key, details) {
                        hits.push(serde_json::json!({
                            "namespace": ns,
                            "route_key": key,
                            "name": details.name,
                            "description": details.description,
                            "example": details.example,
                        }));
                        if hits.len() >= limit {
                            break;
                        }
                    }
                }
            }
        } else {
            let all = self.client.get_all_namespaces().await?;
            'outer: for (ns, routes_map) in all.iter() {
                if let Some(routes) = routes_map.routes.as_ref() {
                    for (key, details) in routes.iter() {
                        if matches(key, details) {
                            hits.push(serde_json::json!({
                                "namespace": ns,
                                "route_key": key,
                                "name": details.name,
                                "description": details.description,
                                "example": details.example,
                            }));
                            if hits.len() >= limit {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        if format.unwrap_or("text").eq_ignore_ascii_case("json") {
            Ok(serde_json::to_string_pretty(&hits)?)
        } else if hits.is_empty() {
            Ok(format!("No route found matching '{query}'."))
        } else {
            let mut lines = Vec::new();
            lines.push(format!(
                "Found {} routes (showing up to {limit}):",
                hits.len()
            ));
            for h in hits.iter() {
                let ns = h.get("namespace").and_then(|v| v.as_str()).unwrap_or("");
                let key = h.get("route_key").and_then(|v| v.as_str()).unwrap_or("");
                let name = h.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let desc = h
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                lines.push(format!(
                    "- {ns} {key} — {name}{}",
                    if desc.is_empty() {
                        "".to_string()
                    } else {
                        format!(" — {desc}")
                    }
                ));
            }
            Ok(lines.join("\n"))
        }
    }

    /// Handle get_route_detail tool call
    async fn handle_get_route_detail(
        &self,
        namespace: &str,
        route_key: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let routes_map = self.client.get_namespace(namespace).await?;
        let Some(routes) = routes_map.routes else {
            return Ok(format!("Namespace '{namespace}' has no routes."));
        };
        if let Some(details) = routes.get(route_key) {
            if format.unwrap_or("text").eq_ignore_ascii_case("json") {
                Ok(serde_json::to_string_pretty(details)?)
            } else {
                Ok(format!("{details:#?}"))
            }
        } else {
            Ok(format!(
                "Route '{route_key}' not found in namespace '{namespace}'."
            ))
        }
    }

    /// Suggest closest route keys within a namespace
    async fn handle_suggest_route_keys(
        &self,
        namespace: &str,
        partial: &str,
        limit: Option<usize>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let limit = limit.unwrap_or(10);
        let routes_map = self.client.get_namespace(namespace).await?;
        let Some(routes) = routes_map.routes else {
            return Ok(format!("Namespace '{namespace}' has no routes."));
        };
        let p = partial.to_lowercase();
        let mut keys: Vec<&String> = routes.keys().collect();
        // Sort by simple heuristic: contains > starts_with > levenshtein-ish length diff
        keys.sort_by_key(|k| {
            let lk = k.to_lowercase();
            let contains = if lk.contains(&p) { 0 } else { 1 };
            let starts = if lk.starts_with(&p) { 0 } else { 1 };
            let len_diff = (lk.len() as isize - p.len() as isize).abs();
            (contains, starts, len_diff)
        });
        let list: Vec<String> = keys.into_iter().take(limit).cloned().collect();
        Ok(format!(
            "Suggested route keys (top {}):\n{}",
            list.len(),
            list.join("\n")
        ))
    }
    /// Create a new RSSHubService with custom configuration
    #[allow(dead_code)]
    pub fn with_config(config: RsshubClientConfig) -> Self {
        let client = Arc::new(RsshubApiClient::new(config));
        Self { client }
    }

    /// Handle get_all_namespaces tool call
    async fn handle_get_all_namespaces(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let namespaces = self.client.get_all_namespaces().await?;
        Ok(format!("{namespaces:#?}"))
    }

    /// Handle get_namespace tool call
    async fn handle_get_namespace(
        &self,
        namespace: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let routes = self.client.get_namespace(namespace).await?;
        if format.unwrap_or("text").eq_ignore_ascii_case("json") {
            Ok(serde_json::to_string_pretty(&routes)?)
        } else {
            Ok(format!("{routes:#?}"))
        }
    }

    /// Handle search_namespaces tool call - More useful than listing all
    async fn handle_search_namespaces(
        &self,
        query: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let all_namespaces = self.client.get_all_namespaces().await?;

        if let Some(search_query) = query {
            // Filter namespaces that match the search query
            let search_lower = search_query.to_lowercase();
            let filtered: Vec<String> = all_namespaces
                .keys()
                .filter(|key| key.to_lowercase().contains(&search_lower))
                .cloned()
                .collect();

            if filtered.is_empty() {
                Ok(format!(
                    "No namespaces found matching '{search_query}'. Available namespaces: {}",
                    all_namespaces
                        .keys()
                        .cloned()
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
            } else {
                Ok(format!(
                    "Namespaces matching '{search_query}':\n{}",
                    filtered.join("\n")
                ))
            }
        } else {
            // Return a concise list of all namespaces
            let namespace_list: Vec<String> = all_namespaces.keys().cloned().collect();
            Ok(format!(
                "Available namespaces ({} total):\n{}",
                namespace_list.len(),
                namespace_list.join(", ")
            ))
        }
    }

    /// Handle get_radar_rules tool call
    async fn handle_get_radar_rules(
        &self,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let rules = self.client.get_all_radar_rules().await?;
        if format.unwrap_or("text").eq_ignore_ascii_case("json") {
            Ok(serde_json::to_string_pretty(&rules)?)
        } else {
            Ok(format!("{rules:#?}"))
        }
    }

    /// Handle get_radar_rule tool call
    async fn handle_get_radar_rule(
        &self,
        rule_name: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let rule = self.client.get_radar_rule(rule_name).await?;
        if format.unwrap_or("text").eq_ignore_ascii_case("json") {
            Ok(serde_json::to_string_pretty(&rule)?)
        } else {
            Ok(format!("{rule:#?}"))
        }
    }

    /// Handle get_categories tool call
    async fn handle_get_categories(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Since get_category requires a parameter, let's get all available categories
        // For now, return a helpful message explaining available categories
        Ok("Available categories: blog, news, programming, social-media, finance, entertainment, government, study, multimedia, picture, travel, shopping, game, reading, university, forecast, bbs, live, anime, tech\n\nUse 'get_category' tool with a specific category name to get feeds for that category.".to_string())
    }

    /// Handle get_category tool call
    async fn handle_get_category(
        &self,
        category: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let category_items = self.client.get_category(category).await?;
        if format.unwrap_or("text").eq_ignore_ascii_case("json") {
            Ok(serde_json::to_string_pretty(&category_items)?)
        } else {
            Ok(format!("{category_items:#?}"))
        }
    }

    /// Handle get_feed tool call - Fetch actual RSS content
    async fn handle_get_feed(
        &self,
        path: &str,
        format: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let feed_response = self.client.get_feed(path).await?;
        if format.unwrap_or("text").eq_ignore_ascii_case("json") {
            Ok(serde_json::to_string_pretty(&feed_response)?)
        } else {
            // Text summary
            let mut lines = Vec::new();
            lines.push(format!("RSS Feed: {}", feed_response.title));
            if !feed_response.description.is_empty() {
                lines.push(format!("Description: {}", feed_response.description));
            }
            let show = feed_response.items.iter().take(3);
            for (idx, item) in show.enumerate() {
                lines.push(format!("- {} {}", idx + 1, item.title));
            }
            if feed_response.raw_content.is_some() {
                lines.push("(raw content available)".to_string());
            }
            Ok(lines.join("\n"))
        }
    }
}

#[async_trait]
impl ToolHandler for RSSHubService {
    async fn list_tools(&self, _request: ListToolsRequest) -> MCPResult<ListToolsResponse> {
        let tools = vec![
            Tool {
                name: "get_all_namespaces".to_string(),
                description: "Get all available namespaces in RSSHub".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_namespace".to_string(),
                description: "Get routes for a specific namespace".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "namespace": {
                            "type": "string",
                            "description": "The namespace to query (e.g., 'bilibili', 'github')"
                        },
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": ["namespace"]
                }),
            },
            Tool {
                name: "search_namespaces".to_string(),
                description: "Search for namespaces by keyword or list all available namespaces"
                    .to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Optional search keyword to filter namespaces (e.g., 'social', 'news')"
                        }
                    },
                    "required": []
                }),
            },
            Tool {
                name: "get_radar_rules".to_string(),
                description: "Get all radar rules for automatic feed detection".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": []
                }),
            },
            Tool {
                name: "get_radar_rule".to_string(),
                description: "Get a specific radar rule by name".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "rule_name": {
                            "type": "string",
                            "description": "The domain/rule name to query (e.g., 'github.com')"
                        },
                        "domain": {
                            "type": "string",
                            "description": "Alias of rule_name; the domain to query (e.g., 'github.com')"
                        },
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": []
                }),
            },
            Tool {
                name: "get_categories".to_string(),
                description:
                    "List known RSSHub categories (informational; use get_category for details)"
                        .to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_category".to_string(),
                description: "Get feeds for a specific category".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "The category name (e.g., 'tech', 'news', 'programming')"
                        },
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": ["category"]
                }),
            },
            Tool {
                name: "get_feed".to_string(),
                description: "Fetch actual RSS feed content from a RSSHub path".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The RSSHub path (e.g., 'bilibili/user/video/2267573', 'github/issue/DIYgod/RSSHub')"
                        },
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": ["path"]
                }),
            },
            Tool {
                name: "search_routes".to_string(),
                description:
                    "Search routes by keyword across all namespaces or within a specific namespace"
                        .to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search keyword (matches key, name, description, example)"},
                        "namespace": {"type": "string", "description": "Optional namespace to restrict search"},
                        "limit": {"type": "integer", "minimum": 1, "maximum": 200, "description": "Max results to return (default 20)"},
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": ["query"]
                }),
            },
            Tool {
                name: "get_route_detail".to_string(),
                description: "Get detailed information for a specific route within a namespace"
                    .to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "namespace": {"type": "string", "description": "The namespace (e.g., 'bilibili')"},
                        "route_key": {"type": "string", "description": "The route key as in namespace map (e.g., '/live/room/:roomID')"},
                        "format": {"type": "string", "enum": ["text", "json"], "description": "Output format (default text)"}
                    },
                    "required": ["namespace", "route_key"]
                }),
            },
            Tool {
                name: "suggest_route_keys".to_string(),
                description: "Suggest closest route keys within a namespace for a partial path"
                    .to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "namespace": {"type": "string", "description": "The namespace (e.g., 'bilibili')"},
                        "partial": {"type": "string", "description": "Partial route key to match (e.g., 'live/room')"},
                        "limit": {"type": "integer", "minimum": 1, "maximum": 50, "description": "Max suggestions to return (default 10)"}
                    },
                    "required": ["namespace", "partial"]
                }),
            },
        ];

        Ok(ListToolsResponse {
            tools,
            next_cursor: None,
        })
    }

    async fn handle_tool_call(&self, request: ToolCallRequest) -> MCPResult<ToolCallResponse> {
        info!(
            "Calling tool: {} with args: {:?}",
            request.name, request.arguments
        );

        let result = match request.name.as_str() {
            "get_all_namespaces" => self.handle_get_all_namespaces().await,
            "get_namespace" => {
                let namespace = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("namespace"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        MCPError::invalid_params("namespace parameter is required".to_string())
                    })?;
                let format = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|v| v.as_str());
                self.handle_get_namespace(namespace, format).await
            }
            "search_namespaces" => {
                let query = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("query"))
                    .and_then(|v| v.as_str());
                self.handle_search_namespaces(query).await
            }
            "get_radar_rules" => {
                let format = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|v| v.as_str());
                self.handle_get_radar_rules(format).await
            }
            "get_radar_rule" => {
                // Accept either rule_name or domain for convenience
                let rule_name_opt = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("rule_name"))
                    .and_then(|v| v.as_str())
                    .or_else(|| {
                        request
                            .arguments
                            .as_ref()
                            .and_then(|args| args.get("domain"))
                            .and_then(|v| v.as_str())
                    });
                let rule_name = rule_name_opt.ok_or_else(|| {
                    MCPError::invalid_params(
                        "rule_name or domain parameter is required".to_string(),
                    )
                })?;
                let format = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|v| v.as_str());
                self.handle_get_radar_rule(rule_name, format).await
            }
            "get_categories" => self.handle_get_categories().await,
            "get_category" => {
                let category = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("category"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        MCPError::invalid_params("category parameter is required".to_string())
                    })?;
                let format = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|v| v.as_str());
                self.handle_get_category(category, format).await
            }
            "get_feed" => {
                let path = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("path"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        MCPError::invalid_params("path parameter is required".to_string())
                    })?;
                let format = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("format"))
                    .and_then(|v| v.as_str());
                self.handle_get_feed(path, format).await
            }
            "search_routes" => {
                let args = request.arguments.as_ref().ok_or_else(|| {
                    MCPError::invalid_params("arguments are required".to_string())
                })?;
                let query = args.get("query").and_then(|v| v.as_str()).ok_or_else(|| {
                    MCPError::invalid_params("query parameter is required".to_string())
                })?;
                let namespace = args.get("namespace").and_then(|v| v.as_str());
                let limit = args
                    .get("limit")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);
                let format = args.get("format").and_then(|v| v.as_str());
                self.handle_search_routes(query, namespace, limit, format)
                    .await
            }
            "get_route_detail" => {
                let args = request.arguments.as_ref().ok_or_else(|| {
                    MCPError::invalid_params("arguments are required".to_string())
                })?;
                let namespace =
                    args.get("namespace")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            MCPError::invalid_params("namespace parameter is required".to_string())
                        })?;
                let route_key =
                    args.get("route_key")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            MCPError::invalid_params("route_key parameter is required".to_string())
                        })?;
                let format = args.get("format").and_then(|v| v.as_str());
                self.handle_get_route_detail(namespace, route_key, format)
                    .await
            }
            "suggest_route_keys" => {
                let args = request.arguments.as_ref().ok_or_else(|| {
                    MCPError::invalid_params("arguments are required".to_string())
                })?;
                let namespace =
                    args.get("namespace")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| {
                            MCPError::invalid_params("namespace parameter is required".to_string())
                        })?;
                let partial = args
                    .get("partial")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        MCPError::invalid_params("partial parameter is required".to_string())
                    })?;
                let limit = args
                    .get("limit")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);
                self.handle_suggest_route_keys(namespace, partial, limit)
                    .await
            }
            _ => {
                return Err(MCPError::method_not_found(format!(
                    "Unknown tool: {}",
                    request.name
                )));
            }
        };

        match result {
            Ok(content) => Ok(ToolCallResponse {
                content: vec![ToolContent::text(content)],
                is_error: Some(false),
            }),
            Err(e) => Ok(ToolCallResponse {
                content: vec![ToolContent::text(format!("Error: {e}"))],
                is_error: Some(true),
            }),
        }
    }
}
