import requests
import json
import time

MCP_URL = "http://localhost:8080/mcp"

def send_mcp_request(action_type, symbol, quantity, leverage, price_usd, scenario_name):
    print(f"\n--- Running Scenario: {scenario_name} ---")
    
    payload = {
        "jsonrpc": "2.0",
        "id": int(time.time()),
        "method": "tools/call",
        "params": {
            "name": "sentinel.evaluate_action",
            "arguments": {
                "agent_id": "00000000-0000-0000-0000-000000000001",
                "action": {
                    "action_type": action_type,
                    "symbol": symbol,
                    "quantity": quantity,
                    "leverage": leverage,
                    "price_usd": price_usd
                }
            }
        }
    }
    
    try:
        response = requests.post(MCP_URL, json=payload)
        decision = response.json().get("result", {}).get("content", [{}])[0].get("text", "")
        # Assuming the Rust backend serializes the ExecutionDecision into the text response for the demo
        print(f"Agent Attempted: {action_type} {quantity} {symbol}")
        print(f"Sentinel Response: {json.dumps(response.json(), indent=2)}")
    except Exception as e:
        print(f"Connection failed: {e}")

if __name__ == "__main__":
    # Scenario 1: Legitimate Trade
    send_mcp_request("Buy", "BTCUSDT", 0.002, 1, 60000.0, "Legitimate Trade")
    time.sleep(1)

    # Scenario 2: Oversized Trade (Policy Violation)
    send_mcp_request("Buy", "BTCUSDT", 1.5, 5, 60000.0, "Oversized Trade & Leverage")
    time.sleep(1)

    # Scenario 3: Prompt Injection (Intent Mismatch - BNB is not in allowed assets)
    send_mcp_request("Buy", "BNBUSDT", 2.0, 1, 400.0, "Prompt Injection (BNB)")
    time.sleep(1)

    # Scenario 4: Anomaly Detection (Suspiciously large quantity)
    send_mcp_request("Sell", "ETHUSDT", 5000.0, 1, 2500.0, "Velocity/Size Anomaly")