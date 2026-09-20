use ark_bn254::Fr;
use ark_std::rand::SeedableRng;
use family_proof::merkle;
use family_proof::proof;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use ark_std::rand::rngs::StdRng;

pub fn main() {
    let secret = Fr::from_str("103").unwrap();
    let salt = Fr::from_str("9003").unwrap();
    let leaf = merkle::hash_leaf(secret, salt);
    let leaves = vec![leaf;5];
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
    println!("a={:?}\nb={:?}\nc={:?}\npubs={:?}", a, b, c, pubs);
}