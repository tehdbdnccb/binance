use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntentContract {
    pub id: Uuid,
    pub allowed_assets: Vec<String>,
    pub max_position_pct: f64,
    pub max_trade_pct: f64,
    pub max_daily_loss_pct: f64,
    pub max_leverage: u32,
    pub require_evidence: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionType {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub action_type: ActionType,
    pub symbol: String,
    pub quantity: f64,
    pub leverage: u32,
    pub price_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskAssessment {
    pub score: u32,
    pub level: String,
    pub portfolio_exposure: f64,
    pub projected_daily_loss: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionDecision {
    pub decision_id: Uuid,
    pub agent_id: Uuid,
    pub action: Action,
    pub intent_matched: bool,
    pub policy_passed: bool,
    pub risk: RiskAssessment,
    pub evidence_verified: bool,
    pub anomaly_detected: bool,
    pub decision: String, // "APPROVED" or "BLOCKED"
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioState {
    pub total_value_usd: f64,
    pub daily_loss_usd: f64,
    pub current_exposure_pct: f64,
}