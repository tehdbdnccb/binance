use crate::models::{Action, ExecutionDecision};

pub struct BinanceClient {
    api_key: String,
    api_secret: String,
}

impl BinanceClient {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("BINANCE_API_KEY").unwrap_or_default(),
            api_secret: std::env::var("BINANCE_SECRET").unwrap_or_default(),
        }
    }

    pub async fn execute_approved_action(&self, decision: &ExecutionDecision) -> Result<String, String> {
        if decision.decision != "APPROVED" {
            return Err("FATAL: Attempted to execute a blocked or unverified decision.".to_string());
        }

        // POST https://api.binance.com/api/v3/order
        // Hackathon stub: Assume execution passes and return a mock Binance Order ID.
        tracing::info!(
            "EXECUTING ON BINANCE: {} {} at ${}", 
            format!("{:?}", decision.action.action_type), decision.action.quantity, decision.action.price_usd
        );
        
        Ok(format!("BINANCE-ORDER-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>()))
    }
}