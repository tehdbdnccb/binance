use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::api::state::AppState;

// --- Decision Center Ledger ---
pub async fn get_decisions(State(state): State<AppState>) -> Json<Value> {
    // In production: sqlx::query!("SELECT * FROM decisions ORDER BY created_at DESC LIMIT 50")
    let decisions = json!([
        {
            "id": "DEC-83921",
            "action": "BUY BTCUSDT",
            "amount": 120.00,
            "intent": "MATCH",
            "policy": "PASS",
            "risk_score": 24,
            "anomaly": "NONE",
            "decision": "APPROVED",
            "timestamp": "2026-09-04T18:22:31Z"
        },
        {
            "id": "DEC-83922",
            "action": "BUY BNBUSDT",
            "amount": 800.00,
            "intent": "MISMATCH",
            "policy": "FAIL",
            "risk_score": 85,
            "anomaly": "DETECTED",
            "decision": "BLOCKED",
            "reason": "Requested exposure 38% exceeds maximum 10%",
            "timestamp": "2026-09-04T18:17:42Z"
        }
    ]);
    Json(decisions)
}

// --- Policy Configuration ---
pub async fn get_policy(State(_state): State<AppState>) -> Json<Value> {
    let policy = json!({
        "contract_id": "IC-001",
        "allowed_assets": ["BTCUSDT", "ETHUSDT"],
        "max_position_pct": 10.0,
        "max_trade_pct": 2.0,
        "max_daily_loss_pct": 3.0,
        "max_leverage": 2,
        "require_evidence": true,
        "status": "ACTIVE"
    });
    Json(policy)
}