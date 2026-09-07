use crate::models::{Action, IntentContract};

pub fn evaluate(action: &Action, contract: &IntentContract) -> Result<(), String> {
    if !contract.allowed_assets.contains(&action.symbol) {
        return Err(format!("Asset {} not permitted by Intent Contract", action.symbol));
    }

    if action.leverage > contract.max_leverage {
        return Err(format!("Requested leverage {}x exceeds maximum {}x", action.leverage, contract.max_leverage));
    }

    Ok(())
}