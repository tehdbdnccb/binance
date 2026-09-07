#[cfg(test)]
mod tests {
    use crate::models::{Action, ActionType, IntentContract, PortfolioState};
    use crate::engines::{policy, risk};
    use uuid::Uuid;

    fn mock_contract() -> IntentContract {
        IntentContract {
            id: Uuid::new_v4(),
            allowed_assets: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
            max_position_pct: 10.0,
            max_trade_pct: 2.0,
            max_daily_loss_pct: 3.0,
            max_leverage: 2,
            require_evidence: true,
        }
    }

    #[test]
    fn test_policy_rejects_unauthorized_asset() {
        let contract = mock_contract();
        let action = Action {
            action_type: ActionType::Buy,
            symbol: "BNBUSDT".to_string(), // Not in allowed_assets
            quantity: 1.0,
            leverage: 1,
            price_usd: 400.0,
        };

        let result = policy::evaluate(&action, &contract);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Asset BNBUSDT not permitted by Intent Contract");
    }

    #[test]
    fn test_risk_engine_calculates_high_risk() {
        let contract = mock_contract();
        let portfolio = PortfolioState {
            total_value_usd: 10_000.0,
            daily_loss_usd: 0.0,
            current_exposure_pct: 8.0,
        };
        
        let action = Action {
            action_type: ActionType::Buy,
            symbol: "BTCUSDT".to_string(),
            quantity: 0.05, // 0.05 * 60k = $3,000 (30% of portfolio, exceeds 2% max_trade)
            leverage: 1,
            price_usd: 60_000.0,
        };

        let assessment = risk::evaluate(&action, &portfolio, &contract);
        assert!(assessment.score >= 50); 
        assert_eq!(assessment.level, "HIGH"); // Or CRITICAL depending on exact algorithm tuning
    }
}