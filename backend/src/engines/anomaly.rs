use crate::models::Action;

pub fn detect_anomaly(action: &Action, recent_trades_count: usize, avg_trade_size: f64) -> Result<(), String> {
    // 1. Velocity Anomaly
    if recent_trades_count > 10 {
        return Err("ANOMALY: High frequency trading threshold exceeded.".to_string());
    }

    // 2. Size Anomaly (e.g., trade is 10x larger than historical baseline)
    let trade_value = action.quantity * action.price_usd;
    if avg_trade_size > 0.0 && trade_value > (avg_trade_size * 10.0) {
        return Err("ANOMALY: Trade size deviates >1000% from behavioral baseline.".to_string());
    }

    Ok(())
}