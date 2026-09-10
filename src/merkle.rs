use ark_bn254::Fr;
use ark_ff::AdditiveGroup;
use pso_poseidon::{Poseidon, PoseidonHasher};

pub type Hash = Fr;

const EMPTY_HASH: Fr = Fr::ZERO;

pub fn hash_leaf(secret: Fr, salt: Fr) -> Hash{
    let mut poseidon = Poseidon::<Fr>::new_circom(3).expect("Error");
    let prefix = Fr::ZERO;
    let hash = poseidon.hash(&[prefix, secret, salt]).unwrap();
    hash
}

pub fn hash_pair(left: &Hash, right: &Hash) -> Hash {
    let mut poseidon = Poseidon::<Fr>::new_circom(2).expect("Error");
    let hash = poseidon.hash(&[*left, *right]).unwrap();
    hash
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree {
    pub layers: Vec<Vec<Hash>>,
}

impl MerkleTree {
    pub fn from_leaves(leaves:Vec<Hash>, levels: usize) -> Self {
        assert!(!leaves.is_empty(),"leaves must not be empty");
        assert!(leaves.len() <= 1 << levels, "leaves length must be less than or equal to 2^levels");
        let mut leaves = leaves;
        let mut leaves_len = leaves.len();
        let mut results = Vec::new();
        let rest = (1 << levels) - leaves_len;
        for _ in 0..rest{
            leaves.push(EMPTY_HASH);
            leaves_len = leaves.len();
        }
        results.push(leaves.clone());
        while leaves_len > 1 {
            let mut result = Vec::new();
            for i in 0..leaves_len/2{
                result.push(hash_pair(&leaves[i * 2], &leaves[i * 2 + 1]));
            }
            results.push(result.clone());
            leaves = result;
            leaves_len = leaves.len();
        }   
    return MerkleTree{layers:results};
    }

    pub fn root(&self) -> Hash {
        self.layers.last().unwrap()[0]
    }

    pub fn depth(&self) -> usize {
        self.layers.len() - 1
    }

    /// `index` 番目の葉から root までの包含証明を作る。
    ///
    /// # Arguments
    /// * `index` - 証明したい葉の位置（`0..葉の数`）
    ///
    /// # Returns
    /// 葉に近い層から順に `(相方のハッシュ, 自分が右の子か)` を並べた `Vec`。
     pub fn proof(&self, index: usize) -> Vec<(Hash, bool)> {
        assert!(index < self.layers[0].len());
        let mut result = Vec::new();
        let mut index = index;
        for i in 0..self.layers.len()-1{
            let mut layer = self.layers[i].clone();
            if layer.len() % 2 == 1{
                layer.push(EMPTY_HASH);
            }
            let is_right = index % 2 == 1;
            if index % 2 == 1{
                result.push((layer[index - 1],is_right))
            } else {
                result.push((layer[index + 1],is_right));
            }
            index = index / 2;
        }
        result
    }
}
    /// 木を再構築せずに、leaf・index・proof・root だけで検証する
    ///
    /// # Arguments
    /// * `leaf` -証明したleaf
    /// * `proof` -leafからrootまでの包含証明
    /// * `root` -merkle treeのroot
    /// # Returns
    /// 含まれるかどうか bool。
pub fn verify_proof(leaf: Hash, proof: Vec<(Hash, bool)>, depth: usize, root: Hash) -> bool {
    if proof.len() != depth{
        return false;
    } 
    let mut leaf = leaf;
    for p in proof{
        let brother_leaf = p.0;
        let is_right = p.1;
        if !is_right{
            leaf = hash_pair(&leaf,&brother_leaf);
        }else{
            leaf = hash_pair(&brother_leaf,&leaf);
        }
    }
    leaf == root
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_leaves(){
        let leaves = vec![hash_leaf(b"abc");6];
        let tree1 = MerkleTree::from_leaves(leaves);
        let a = hash_leaf(b"abc");
        let b = hash_pair(&a,&a);
        let c = hash_pair(&b,&b);
        let e = hash_pair(&b, &EMPTY_HASH);
        let d = hash_pair(&c,&e);
        // let d = hash_pair(&c, &c);
        println!("{:?}",tree1);
        assert_eq!(tree1.layers.last().unwrap()[0],d);
    }
    #[test]
    fn test_root(){
        let leaves = vec![hash_leaf(b"abc");6];
        let tree1 = MerkleTree::from_leaves(leaves);
        let a = hash_leaf(b"abc");
        let b = hash_pair(&a,&a);
        let c = hash_pair(&b,&b);
        let e = hash_pair(&b, &EMPTY_HASH);
        let d = hash_pair(&c,&e);
        println!("{:?}",tree1);
        assert_eq!(tree1.root(),d);
    }

    #[test]
    fn test_proof_verify(){
        let mut leaves = Vec::new();
        for i in 0..7 {
            leaves.push(hash_leaf(format!("{}",i).as_bytes()));
        }
        let tree = MerkleTree::from_leaves(leaves.clone());
        for i in 0..7{
            assert!(verify_proof(leaves[i], tree.proof(i),tree.depth(),tree.root()))
        }
        assert!(!verify_proof(hash_leaf(b"out"),tree.proof(0),tree.depth(), tree.root()));
        assert!(!verify_proof(hash_leaf(b"out"),tree.proof(1),1, tree.root()));
    }

}
