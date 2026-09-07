use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::api::state::AppState;
use crate::mcp::protocol::{McpRequest, McpResponse, McpError};

pub async fn handle_mcp_request(
    State(state): State<AppState>,
    Json(payload): Json<McpRequest>,
) -> Json<McpResponse> {
    match payload.method.as_str() {
        "tools/list" => Json(McpResponse {
            jsonrpc: "2.0".to_string(),
            id: payload.id,
            error: None,
            result: Some(json!({
                "tools": [
                    {
                        "name": "sentinel.evaluate_action",
                        "description": "Submit a proposed financial action to the Sentinel firewall for policy, risk, and intent evaluation.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "action_type": { "type": "string", "enum": ["Buy", "Sell"] },
                                "symbol": { "type": "string" },
                                "quantity": { "type": "number" },
                                "leverage": { "type": "number" },
                                "price_usd": { "type": "number" }
                            },
                            "required": ["action_type", "symbol", "quantity", "price_usd"]
                        }
                    }
                ]
            })),
        }),
        "tools/call" => {
            let params = payload.params.unwrap();
            if params.name.as_deref() == Some("sentinel.evaluate_action") {
                // In a full implementation, route this payload to `evaluate_execution` 
                // from src/engines/mod.rs, wait for the decision, and return the ExecutionDecision.
                Json(McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: payload.id,
                    error: None,
                    result: Some(json!({ "content": [{ "type": "text", "text": "Action evaluated." }] })),
                })
            } else {
                Json(McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: payload.id,
                    result: None,
                    error: Some(McpError { code: -32601, message: "Method not found".to_string() }),
                })
            }
        },
        _ => Json(McpResponse {
            jsonrpc: "2.0".to_string(),
            id: payload.id,
            result: None,
            error: Some(McpError { code: -32600, message: "Invalid Request".to_string() }),
        }),
    }
}