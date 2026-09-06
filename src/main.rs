use rand::RngExt;

use crate::merkle::MerkleTree;

mod merkle;


fn main() {
    let mut rng = rand::rng();
    let secrets:[u8;32] = rng.random();
    let leaves = secrets.map(|s| merkle::hash_leaf(&s.to_le_bytes()));
    let tree = MerkleTree::from_leaves(leaves.to_vec());
    let root = tree.root();
    let not_member = merkle::hash_leaf(b"abc");
        
    for i in 0..leaves.len(){
        let proof = tree.proof(i);
        let verify =merkle::verify_proof(leaves[i], proof.clone(), root);
        assert!(verify);
        let verify2 = merkle::verify_proof(not_member, proof, root);
        assert!(!verify2);
    }
}
