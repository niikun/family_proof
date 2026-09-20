use std::collections::HashMap;
use std::fs;
use ark_bn254::Fr;
use std::str::FromStr;
use alloy::providers::{Provider,ProviderBuilder};
use alloy::rpc::types::Filter;
use alloy::primitives::Address;
use alloy::sol;
use alloy::sol_types::SolEvent;
use tokio;
use family_proof::proof;
use family_proof::merkle;

#[tokio::main]
pub async fn main(){
    dotenvy::dotenv().ok();
    let api_key = std::env::var("RESEND_API_KEY").unwrap();
    let to = std::env::var("NOTIFY_TO").unwrap();

    sol! {
    event ProofVerified(uint256 nullifier, uint256 epoch);
    event PotentialLeak(uint256 indexed nullifier, uint256 challenge1, uint256 y1, uint256 challenge2, uint256 y2, uint256 epoch);
    }

    let registry_address:Address = "0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d".parse().unwrap();
    let url= "https://worldchain-sepolia.g.alchemy.com/public".parse().unwrap();
    let provider = ProviderBuilder::new().connect_http(url);
    let mut from_block: u64 = 34672100;
    let mut stats: HashMap<u64,(u64, u64)> = fs::read_to_string("stats.json")
                                .ok()
                                .and_then(|s| serde_json::from_str(&s).ok())
                                .unwrap_or_default();
    loop {
        let latest = provider.get_block_number().await.unwrap();
        if latest >= from_block {
            let to_block = std::cmp::min(from_block + 99, latest); // 100ブロック幅

            let filter = Filter::new()
                .address(registry_address)
                .from_block(from_block)
                .to_block(to_block);
            let logs = provider.get_logs(&filter).await.unwrap();
            let client = reqwest::Client::new();
            for log in &logs {
                let day = log.block_timestamp.unwrap() / 86400;
                let entry = stats.entry(day).or_insert((0,0));
                if log.topics()[0] == ProofVerified::SIGNATURE_HASH{
                    entry.0 += 1;
                } else if log.topics()[0] == PotentialLeak::SIGNATURE_HASH{
                    entry.1 += 1;
                
                    let decoded = PotentialLeak::decode_log(&log.inner).unwrap();
                    let challenge1 = Fr::from_str(&decoded.challenge1.to_string()).unwrap();
                    let challenge2 = Fr::from_str(&decoded.challenge2.to_string()).unwrap();
                    let y1 = Fr::from_str(&decoded.y1.to_string()).unwrap();
                    let y2 = Fr::from_str(&decoded.y2.to_string()).unwrap();
                    let x1 = merkle::hash_single(challenge1);
                    let x2 = merkle::hash_single(challenge2);
                    let recovered = proof::recover_secret(x1, y1, x2, y2);
                    let body = serde_json::json!({
                        "from": "onboarding@resend.dev",
                        "to": [to],
                        "subject": "FamilyProof: 漏洩の疑いを検知しました",
                        "text": format!("secretの使い回しを検知しました。復元されたsecret: {}", recovered)
                    });
                    let res = client.post("https://api.resend.com/emails")
                                .bearer_auth(&api_key)
                                .json(&body)
                            .send()
                            .await
                            .unwrap();
                    println!("recovered secret = {}", recovered);
                    println!("resend status: {}", res.status());
                    println!("ProofVerified:{:?},\nPotentialLeak:{:?}",entry.0, entry.1);
                }
            }
            fs::write("stats.json",serde_json::to_string_pretty(&stats).unwrap()).unwrap();
            from_block = to_block + 1;
        }
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        
    }    
}