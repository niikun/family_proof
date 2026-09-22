use std::process::Command;
use std::env;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use alloy::primitives::keccak256;

pub fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 3);
    let description = &args[1];
    let tier_num:u32 = args[2].parse::<u32>().expect("Invalid args");
    let tier = tier_num.to_string();
    let description_hashed: alloy::primitives::FixedBytes<32> = keccak256(description.as_bytes());
    let action_id = Fr::from_le_bytes_mod_order(description_hashed.as_slice());  
    let action_id_str = action_id.into_bigint().to_string();
    println!("action_id={}", action_id_str);

    let status = Command::new("cast")
        .args([
            "send",
            "0xf7f344E9399638b69DF158877F1e77a39A5F3D73",
            "proposeAction(uint256 actionId, uint256 tier)",
            &action_id_str,
            &tier,
            "--rpc-url", "https://worldchain-sepolia.g.alchemy.com/public",
            // "--rpc-url", "http://127.0.0.1:8545",  //local test 環境
            "--account", "agent",
        ])
        .status()
        .expect("failed to spawn cast");

    if !status.success() {
        eprintln!("cast send failed: {:?}", status);
    }
    
}