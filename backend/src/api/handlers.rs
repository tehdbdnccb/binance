use axum::{extract::{State, Path}, Json};
use serde_json::{Value, json};
use uuid::Uuid;
use crate::{
    api::state::AppState,
    models::{Action, IntentContract, PortfolioState, ExecutionDecision},
    engines::evaluate_execution,
};

// --- MCP / Execution Endpoint ---
pub async fn evaluate_action(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<ExecutionDecision> {
    // In a real scenario, this is extracted from the MCP payload/auth
    let agent_id = Uuid::parse_str(payload["agent_id"].as_str().unwrap_or_default()).unwrap_or_else(|_| Uuid::new_v4());
    
    let action: Action = serde_json::from_value(payload["action"].clone()).unwrap();

    // Fetch from DB (Mocked for hackathon speed in this handler)
    let contract = IntentContract {
        id: Uuid::new_v4(),
        allowed_assets: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
        max_position_pct: 10.0,
        max_trade_pct: 2.0,
        max_daily_loss_pct: 3.0,
        max_leverage: 2,
        require_evidence: true,
    };

    let portfolio = PortfolioState {
        total_value_usd: 10000.0,
        daily_loss_usd: 50.0,
        current_exposure_pct: 4.0,
    };

    // O(1) Kill switch check via RwLock
    let statuses = state.agent_status.read().await;
    let is_killed = statuses.get(&agent_id).copied().unwrap_or(false);
    drop(statuses);

    let decision = evaluate_execution(agent_id, action, &contract, &portfolio, is_killed);

    // Async write to Audit Ledger (Fire and forget or await depending on strictness)
    let decision_clone = decision.clone();
    tokio::spawn(async move {
        // Insert into DB...
        let _ = sqlx::query!(
            "INSERT INTO decisions (id, agent_id, action_type, symbol, quantity, intent_passed, policy_passed, risk_score, anomaly_detected, final_decision, reason) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            decision_clone.decision_id, decision_clone.agent_id, 
            format!("{:?}", decision_clone.action.action_type), decision_clone.action.symbol, 
            decision_clone.action.quantity, decision_clone.intent_matched, 
            decision_clone.policy_passed, decision_clone.risk.score as i32, 
            decision_clone.anomaly_detected, decision_clone.decision, 
            decision_clone.reason
        )
        .execute(&state.db)
        .await;
    });

    // If APPROVED -> Execute on Binance here via `src/services/binance.rs`

    Json(decision)
}

// --- Dashboard Kill Switch ---
pub async fn toggle_kill_switch(
    State(state): State<AppState>,
    Path(agent_id): Path<Uuid>,
) -> Json<Value> {
    let mut statuses = state.agent_status.write().await;
    let current = statuses.entry(agent_id).or_insert(false);
    *current = !*current;
    
    // Also update DB asynchronously...
    
    Json(json!({
        "agent_id": agent_id,
        "status": if *current { "SUSPENDED" } else { "ACTIVE" }
    }))
}

// --- Dashboard Stats ---
pub async fn get_dashboard(State(state): State<AppState>) -> Json<Value> {
    // Read from DB (Mocked structure matching your prompt)
    Json(json!({
        "protection_status": "ACTIVE",
        "portfolio": 2430.22,
        "risk_utilization": 42,
        "pnl_today": 24.18,
        "recent_decisions": [
            {"action": "BUY BTC", "status": "APPROVED"},
            {"action": "BUY BNB", "status": "BLOCKED"}
        ]
    }))
}