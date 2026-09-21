use ark_bn254::Fr;
use std::str::FromStr;
use family_proof::merkle;

fn main() {
    let old_leaf = merkle::hash_leaf(
        Fr::from_str("103").unwrap(), Fr::from_str("9003").unwrap()
    );
    let mut leaves = vec![old_leaf;5];

    let new_secret = Fr::from_str("203").unwrap();
    let new_salt = Fr::from_str("9203").unwrap();
    leaves[0] = merkle::hash_leaf(new_secret, new_salt);
    let tree = merkle::MerkleTree::from_leaves(leaves, 4);
    println!("new root = {:?}", tree.root());
}