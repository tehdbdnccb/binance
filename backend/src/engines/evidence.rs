use crate::models::IntentContract;

pub struct EvidenceSource {
    pub mcp_name: String,
    pub verified: bool,
    pub trust_score: u32,
}

pub fn verify_evidence(contract: &IntentContract, sources: Vec<EvidenceSource>) -> Result<(), String> {
    if !contract.require_evidence {
        return Ok(());
    }

    if sources.is_empty() {
        return Err("EVIDENCE_MISSING: Policy requires external verifiable evidence for trade.".to_string());
    }

    let all_verified = sources.iter().all(|s| s.verified && s.trust_score > 50);
    
    if !all_verified {
         return Err("EVIDENCE_REJECTED: Untrusted or unverified external MCP source detected.".to_string());
    }

    Ok(())
}