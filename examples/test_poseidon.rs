use ark_bn254::Fr;
use ark_ff::AdditiveGroup;
use pso_poseidon::{Poseidon, PoseidonHasher};
use core::str::FromStr;
// use num_traits::identities::Zero;

const EMPTY_HASH: Fr = Fr::ZERO;

const MEMBERS :[(&str, &str); 5] = [
  ("101", "9001"),
  ("102", "9002"),
  ("103", "9003"),
  ("104", "9004"),
  ("105", "9005")
];

const LEVEL: usize = 4;

pub fn main() {

    let mut poseidon = Poseidon::<Fr>::new_circom(3).expect("Error");
    let mut leaves = Vec::new();
    for member in MEMBERS.iter() {
        let prefix = Fr::from_str("0").unwrap();
        let secret = Fr::from_str(member.0).unwrap();
        let salt = Fr::from_str(member.1).unwrap();
        let leaf = poseidon.hash(&[prefix, secret, salt]).unwrap();
        leaves.push(leaf);
    }
    for _ in 0..11{
        leaves.push(EMPTY_HASH);
    }
    let mut poseidon_2 = Poseidon::<Fr>::new_circom(2).expect("Error");
    let mut tree = Vec::new();
    tree.push(leaves);
    for i in 0..LEVEL{
        let mut layer = Vec::new();
        if tree[i].len() % 2 == 1 {
            tree[i].push(EMPTY_HASH);
        }
        for j in 0..tree[i].len() / 2 {
            let left = tree[i][j * 2];
            let right = tree[i][j * 2 + 1];
            let parent = poseidon_2.hash(&[left, right]).unwrap();
            layer.push(parent);
        }
        tree.push(layer);
    }
    let root = tree.last().unwrap()[0];
    let expected = Fr::from_str("17396252260025783793058854431926620863655419045074533465745990270806947938816").unwrap();

    assert_eq!(root, expected);
}