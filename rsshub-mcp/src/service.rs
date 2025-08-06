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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let routes = self.client.get_namespace(namespace).await?;
        Ok(format!("{routes:#?}"))
    }

    /// Handle get_radar_rules tool call
    async fn handle_get_radar_rules(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let rules = self.client.get_all_radar_rules().await?;
        Ok(format!("{rules:#?}"))
    }

    /// Handle get_radar_rule tool call
    async fn handle_get_radar_rule(
        &self,
        rule_name: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let rule = self.client.get_radar_rule(rule_name).await?;
        Ok(format!("{rule:#?}"))
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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let category_items = self.client.get_category(category).await?;
        Ok(format!("{category_items:#?}"))
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
                        }
                    },
                    "required": ["namespace"]
                }),
            },
            Tool {
                name: "get_radar_rules".to_string(),
                description: "Get all radar rules for automatic feed detection".to_string(),
                annotations: None,
                output_schema: None,
                input_schema: json!({
                    "type": "object",
                    "properties": {},
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
                            "description": "The name of the radar rule to query"
                        }
                    },
                    "required": ["rule_name"]
                }),
            },
            Tool {
                name: "get_categories".to_string(),
                description: "Get all available categories in RSSHub".to_string(),
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
                        }
                    },
                    "required": ["category"]
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
                self.handle_get_namespace(namespace).await
            }
            "get_radar_rules" => self.handle_get_radar_rules().await,
            "get_radar_rule" => {
                let rule_name = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("rule_name"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        MCPError::invalid_params("rule_name parameter is required".to_string())
                    })?;
                self.handle_get_radar_rule(rule_name).await
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
                self.handle_get_category(category).await
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
