use crate::models::{Action, IntentContract, PortfolioState, RiskAssessment};

pub fn evaluate(action: &Action, portfolio: &PortfolioState, contract: &IntentContract) -> RiskAssessment {
    let trade_value_usd = action.quantity * action.price_usd * (action.leverage as f64);
    let trade_pct = (trade_value_usd / portfolio.total_value_usd) * 100.0;
    
    let projected_exposure = portfolio.current_exposure_pct + trade_pct;
    
    // Simple MVP risk scoring algorithm
    let mut score = 0;
    
    if trade_pct > contract.max_trade_pct {
        score += 50;
    }
    if projected_exposure > contract.max_position_pct {
        score += 40;
    }
    
    let level = match score {
        0..=30 => "LOW",
        31..=60 => "MEDIUM",
        61..=80 => "HIGH",
        _ => "CRITICAL",
    };

    RiskAssessment {
        score,
        level: level.to_string(),
        portfolio_exposure: projected_exposure,
        projected_daily_loss: portfolio.daily_loss_usd, 
    }
}