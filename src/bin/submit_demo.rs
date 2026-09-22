use std::process::Command;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_std::rand::SeedableRng;
use family_proof::merkle;
use family_proof::proof;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use ark_std::rand::rngs::StdRng;

pub fn main() {
    let old_secret = Fr::from_str("103").unwrap();
    let old_salt = Fr::from_str("9003").unwrap();
    let old_leaf = merkle::hash_leaf(old_secret, old_salt);
    let mut leaves = vec![old_leaf; 5];   // まずindex 0〜4を旧secretで埋める

    let secret = Fr::from_str("203").unwrap();
    let salt = Fr::from_str("9203").unwrap();
    leaves[0] = merkle::hash_leaf(secret, salt);   // index 0だけ新secretで上書き

    let tree = merkle::MerkleTree::from_leaves(leaves, 4);
    let proof = tree.proof(0);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let epoch = Fr::from(now / 3600);
    let args: Vec<String> = std::env::args().collect();
    let challenge_str = args.get(1).map(String::as_str).unwrap_or("777");
    let challenge = Fr::from_str(challenge_str).unwrap();
    let siblings:Vec<Fr> = proof.iter().map(|p| p.0).collect();
    let path_indices: Vec<bool> = proof.iter().map(|p| p.1).collect();
    let circuits = proof::build_circuit_with_inputs(secret, salt, epoch, challenge, &path_indices, &siblings).unwrap();
    let (pk, _pkv) = proof::setup().unwrap();
    let mut rng =StdRng::seed_from_u64(0);
    let (pf,pubs) = proof::prove(&pk, circuits, &mut rng).unwrap();
    let (a, b, c) = proof::to_solidity_calldata(pf);
    // println!("a={:?}\nb={:?}\nc={:?}\npubs={:?}", a, b, c, pubs);

    let a_arg = format!("[{}]",a.join(","));
    let b_arg = format!("[[{}],[{}]]",b[0].join(","), b[1].join(","));
    let c_arg = format!("[{}]",c.join(","));
    let pubs_arg = format!(
        "[{}]",
        pubs.iter()
            .map(|p| p.into_bigint().to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    let status = Command::new("cast")
    .args([
        "send",
        "0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d",
        "verifyMembership(uint256[2],uint256[2][2],uint256[2],uint256[5])",
        &a_arg,
        &b_arg,
        &c_arg,
        &pubs_arg,
        "--rpc-url", "https://worldchain-sepolia.g.alchemy.com/public",
        "--account", "deployer",
    ])
    .status()
    .expect("failed to spawn cast"); 
    if status.success() {
        println!("############################ Membership Verified #############################");
        println!("");
        println!("challenge: {}", challenge_str);
        let nullifier_full = pubs[2].into_bigint().to_string();
        let nullifier_short = format!("{}...{}", &nullifier_full[..8], &nullifier_full[nullifier_full.len()-6..]);
        println!("nullifier: {}", nullifier_short);
        println!("");
        println!("##############################################################################");
        eprintln!("cast send failed: {:?}", status);
    }
}