CREATE TABLE agents (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    trust_score INT NOT NULL DEFAULT 100,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE policies (
    id UUID PRIMARY KEY,
    agent_id UUID REFERENCES agents(id),
    name VARCHAR(255) NOT NULL,
    allowed_assets JSONB NOT NULL,
    max_position_pct FLOAT NOT NULL,
    max_trade_pct FLOAT NOT NULL,
    max_daily_loss_pct FLOAT NOT NULL,
    max_leverage INT NOT NULL,
    require_evidence BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE decisions (
    id UUID PRIMARY KEY,
    agent_id UUID REFERENCES agents(id),
    action_type VARCHAR(50) NOT NULL,
    symbol VARCHAR(50) NOT NULL,
    quantity FLOAT NOT NULL,
    intent_passed BOOLEAN NOT NULL,
    policy_passed BOOLEAN NOT NULL,
    risk_score INT NOT NULL,
    anomaly_detected BOOLEAN NOT NULL,
    final_decision VARCHAR(50) NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);