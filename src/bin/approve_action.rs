use std::process::Command;
use std::env;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use alloy::primitives::keccak256;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use family_proof::merkle;
use family_proof::proof;

pub fn main() {
    let args:Vec<String> = env::args().collect();
    let challenge_str = args.get(1).map(String::as_str).unwrap_or("777");
    let leaf_idx = args.get(2).unwrap().parse::<usize>().unwrap();

    let old_secret = Fr::from_str("103").unwrap();
    let old_salt = Fr::from_str("9003").unwrap();
    let old_leaf = merkle::hash_leaf(old_secret, old_salt);
    let mut leaves = vec![old_leaf; 5];   // まずindex 0〜4を旧secretで埋める

    let new_secret = Fr::from_str("203").unwrap();
    let new_salt = Fr::from_str("9203").unwrap();
    leaves[0] = merkle::hash_leaf(new_secret, new_salt);   // index 0だけ新secretで上書き
    let mut secret = new_secret;
    let mut salt = new_salt;
    if leaf_idx == 1 {
        secret = old_secret;
        salt = old_salt;
    } 

    let tree = merkle::MerkleTree::from_leaves(leaves, 4);
    let proof = tree.proof(leaf_idx);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let epoch = Fr::from(now / 3600);

    let challenge_hashed = keccak256(challenge_str.as_bytes());
    let challenge = Fr::from_le_bytes_mod_order(challenge_hashed.as_slice());
    let siblings:Vec<Fr> = proof.iter().map(|p| p.0).collect();
    let path_indices:Vec<bool> = proof.iter().map(|p| p.1).collect();
    let circuit = proof::build_circuit_with_inputs(
        secret, salt, epoch, challenge, &path_indices, &siblings
    ).unwrap();
    let (pk, _) = proof::setup().unwrap();
    let mut rng = StdRng::seed_from_u64(0);
    let (pf, pubs) = proof::prove(&pk, circuit, &mut rng).unwrap();
    let (a, b, c) = proof::to_solidity_calldata(pf);
    // println!("a={:?}\nb={:?}\nc={:?}\npubs={:?}", a, b, c, pubs);
    let challenge_arg = format!("{:?}",challenge);
    let a_arg = format!("[{}]", a.join(","));
    let b_arg = format!("[[{}],[{}]]",b[0].join(","), b[1].join(","));
    let c_arg = format!("[{}]",c.join(","));
    let pubs_arg = format!(
        "[{}]",
        pubs.iter()
            .map(|p| p.into_bigint().to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let output = Command::new("cast")
        .args([
            "send",
            "0xf7f344E9399638b69DF158877F1e77a39A5F3D73",
            "approveAction(uint256,uint256[2],uint256[2][2],uint256[2],uint256[5])",
            &challenge_arg,
            &a_arg,
            &b_arg,
            &c_arg,
            &pubs_arg,
            "--rpc-url", "https://worldchain-sepolia.g.alchemy.com/public",
            "--account", "agent",  
        ])
        .output()
        .expect("failed to spawn cast");

    print!("{}", String::from_utf8_lossy(&output.stdout));
    
    let authorized = String::from_utf8_lossy(&output.stdout)
    .contains("0xb402c6ca06ec77e392ffe0828856a7aa022bea631ef0a4a96359d6dfc3e9b9d0");

    if output.status.success(){
        println!("############################## Action Approved ##############################");
        println!("");
        println!("action: {} is approved by idx-{}",challenge_str,leaf_idx);
        println!("action_id={}", pubs[4]);
        println!("");
        println!("##############################################################################");
        if authorized {
        println!("🎉 ActionAuthorized!");
        } else {
            println!("(pending — threshold not yet reached)");
        }

    } else {
        eprint!("cast send failed: {:?}", output.status);
    }

}