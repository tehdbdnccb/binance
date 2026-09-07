use crate::models::{Action, IntentContract};

pub fn verify_intent(action: &Action, contract: &IntentContract) -> Result<(), String> {
    // Prevent the agent from trading assets missing from the user's explicit natural language request
    if !contract.allowed_assets.contains(&action.symbol) {
        return Err("INTENT_MISMATCH: Asset not authorized in user intent contract.".to_string());
    }
    
    // Additional MVP logic for detecting prompt-injected instructions bypassing intent
    if action.quantity <= 0.0 {
        return Err("INTENT_MISMATCH: Quantity must be positive.".to_string());
    }

    Ok(())
}