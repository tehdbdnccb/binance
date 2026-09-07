pub mod policy;
pub mod risk;

use crate::models::{Action, ExecutionDecision, IntentContract, PortfolioState, RiskAssessment};
use chrono::Utc;
use uuid::Uuid;

pub fn evaluate_execution(
    agent_id: Uuid,
    action: Action,
    contract: &IntentContract,
    portfolio: &PortfolioState,
    agent_is_killed: bool,
) -> ExecutionDecision {
    let mut decision = ExecutionDecision {
        decision_id: Uuid::new_v4(),
        agent_id,
        action: action.clone(),
        intent_matched: true,
        policy_passed: true,
        risk: RiskAssessment {
            score: 0,
            level: "LOW".to_string(),
            portfolio_exposure: 0.0,
            projected_daily_loss: 0.0,
        },
        evidence_verified: true,
        anomaly_detected: false,
        decision: "APPROVED".to_string(),
        reason: None,
        timestamp: Utc::now(),
    };

    // 1. Kill Switch Check (O(1) memory check in handler, but enforced here)
    if agent_is_killed {
        decision.decision = "BLOCKED".to_string();
        decision.reason = Some("AGENT_SUSPENDED".to_string());
        return decision;
    }

    // 2. Intent & Policy Check
    if let Err(reason) = policy::evaluate(&action, contract) {
        decision.policy_passed = false;
        decision.intent_matched = false;
        decision.decision = "BLOCKED".to_string();
        decision.reason = Some(reason);
        return decision;
    }

    // 3. Risk Check
    let risk_eval = risk::evaluate(&action, portfolio, contract);
    decision.risk = risk_eval.clone();
    
    if risk_eval.score > 80 {
        decision.decision = "BLOCKED".to_string();
        decision.reason = Some("CRITICAL_RISK_EXCEEDED".to_string());
        return decision;
    }

    // 4. Anomaly Check (Hackathon stub: block if qty is suspiciously round and massive)
    if action.quantity > 1000.0 {
        decision.anomaly_detected = true;
        decision.decision = "BLOCKED".to_string();
        decision.reason = Some("ANOMALOUS_TRADE_SIZE".to_string());
    }

    decision
}