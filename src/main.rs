use rand::RngExt;
use ark_bn254::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_ff::PrimeField;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::proof::to_solidity_calldata;
mod merkle;
mod proof;

const FAMILY_MEMBERS:u8 = 7;

fn main() ->color_eyre::Result<()>{
    color_eyre::install()?;
    let mut rng = rand::rng();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let epoch = Fr::from(now / 3600);
    let challenge = Fr::from(777u64);

    let mut leaves = Vec::new();
    let mut members:Vec<(Fr, Fr)> = Vec::new();
    for _ in 0..FAMILY_MEMBERS {
        let secret_bytes:[u8;32] = rng.random();
        let salt_bytes:[u8;32] = rng.random();
        let secret = Fr::from_le_bytes_mod_order(&secret_bytes);
        let salt = Fr::from_le_bytes_mod_order(&salt_bytes);
        leaves.push(merkle::hash_leaf(secret, salt));
        members.push((secret, salt));
    }
    let tree = merkle::MerkleTree::from_leaves(leaves.clone(),4);
    let depth = tree.depth();
    let root = tree.root();
    let not_member = merkle::hash_leaf(Fr::from(999u64), Fr::from(999u64));
    let mut std_rng = StdRng::seed_from_u64(42);
    let (pk, pvk) = proof::setup()?;
    for i in 0..leaves.len(){
        let proof = tree.proof(i);
        let mut siblings:Vec<Fr> = Vec::new();
        let mut path_indices:Vec<bool> = Vec::new();
        proof.iter().for_each(|(s,p)| {
            siblings.push(*s);
            path_indices.push(*p);
        });
        let (secret, salt) = members[i];
        let circuit = proof::build_circuit_with_inputs(secret, salt, epoch, challenge, &path_indices, &siblings)?;
        let (pf, pubs) = proof::prove(&pk, circuit, &mut std_rng)?;
        assert_eq!(pubs[0], root);
        let (a,b,c) = to_solidity_calldata(pf.clone());
        let ok = proof::verify(&pvk, &pubs, &pf)?;
        println!("verify = {}", ok);
        
        let verify =merkle::verify_proof(leaves[i], proof.clone(), depth, root);
        assert!(verify);
        let verify2 = merkle::verify_proof(not_member, proof, depth, root);
        assert!(!verify2);
    }
    Ok(())
}

