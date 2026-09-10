use rand::RngExt;
use ark_bn254::Fr;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_ff::PrimeField;
mod merkle;
mod proof;

const FAMILY_MEMBERS:u8 = 7;

fn main() ->color_eyre::Result<()>{
    color_eyre::install()?;
    let mut rng = rand::rng();

    let mut leaves = Vec::new();
    for _ in 0..FAMILY_MEMBERS {
        let secret_bytes:[u8;32] = rng.random();
        let salt_bytes:[u8;32] = rng.random();
        let secret = Fr::from_le_bytes_mod_order(&secret_bytes);
        let salt = Fr::from_le_bytes_mod_order(&salt_bytes);
        leaves.push(merkle::hash_leaf(secret, salt));
    }
    let tree = merkle::MerkleTree::from_leaves(leaves.clone(),4);
    let depth = tree.depth();
    let root = tree.root();
    let not_member = merkle::hash_leaf(Fr::from(999u64), Fr::from(999u64));
        
    for i in 0..leaves.len(){
        let proof = tree.proof(i);
        let verify =merkle::verify_proof(leaves[i], proof.clone(), depth, root);
        assert!(verify);
        let verify2 = merkle::verify_proof(not_member, proof, depth, root);
        assert!(!verify2);
    }
    
    let mut std_rng = StdRng::seed_from_u64(42);
    let circuit = proof::build_circuit()?;
    let (pk, pvk) = proof::setup(circuit.clone(), &mut std_rng)?;
    let (pf, pubs) = proof::prove(&pk, circuit, &mut std_rng)?;
    let ok = proof::verify(&pvk, &pubs, &pf)?;
    println!("verify = {}", ok);
    

    Ok(())
}
