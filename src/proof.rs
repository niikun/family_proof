
use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomCircuit, CircomConfig, CircomBuilder, read_zkey};
use ark_groth16::{Groth16, Proof, ProvingKey, PreparedVerifyingKey};
use ark_snark::SNARK;
use ark_std::rand::rngs::StdRng;
use ark_ff::{PrimeField, Field};

use std::fs::File;

pub fn build_circuit() -> color_eyre::Result<CircomCircuit<Fr>>{
    let cfg = CircomConfig::<Fr>::new(
        "circuits/main_js/main.wasm",
        "circuits/main.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.load_input_json("circuits/input.json")?;  
    Ok(builder.build()?)
}

pub fn build_circuit_with_inputs(
    secret: Fr,
    salt:Fr,
    epoch:Fr,
    challenge:Fr,
    path_indices: &[bool],
    siblings: &[Fr],
) -> color_eyre::Result<CircomCircuit<Fr>>{
   let cfg = CircomConfig::<Fr>::new(
        "circuits/main_js/main.wasm",
        "circuits/main.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("secret", secret.into_bigint());
    builder.push_input("salt", salt.into_bigint());
    builder.push_input("epoch", epoch.into_bigint());
    builder.push_input("challenge", challenge.into_bigint());
    for i in 0..4 {
        builder.push_input("pathIndices",path_indices[i] as u64);
        builder.push_input("siblings", siblings[i].into_bigint());
    }
    Ok(builder.build()?)
}

pub fn setup() -> color_eyre::Result<(ProvingKey<Bn254>, PreparedVerifyingKey<Bn254>)> {
    let mut file = File::open("circuits/main_final.zkey")?;
    let (pk, _matrices) = read_zkey(&mut file)?;
    let pvk = Groth16::<Bn254>::process_vk(&pk.vk)?;
    Ok((pk, pvk))
}

pub fn prove(pk: &ProvingKey<Bn254>, circuit: CircomCircuit<Fr>, rng: &mut StdRng)
    -> color_eyre::Result<(Proof<Bn254>, Vec<Fr>)> {
    // circuit は prove で消費されるので、public inputs を先に取る
    let public_inputs = circuit.get_public_inputs().unwrap();
    let proof = Groth16::<Bn254, ark_circom::CircomReduction>::prove(pk, circuit, rng)?;
    Ok((proof, public_inputs))
}

pub fn verify(pvk: &PreparedVerifyingKey<Bn254>, public_inputs: &[Fr], proof: &Proof<Bn254>)
    -> color_eyre::Result<bool> {
    Ok(Groth16::<Bn254>::verify_with_processed_vk(pvk, public_inputs, proof)?)
}

pub fn recover_secret(x1: Fr, y1: Fr, x2: Fr, y2: Fr) -> Fr {
    let a1 = (y2 - y1) * (x2 - x1).inverse().unwrap();
    y1 - a1 * x1
}

#[cfg(test)]
mod test{
    use super::*;
    use ark_std::rand::SeedableRng;
    use std::str::FromStr;

    #[test]
    fn test_build_circuit(){
        let circuit = build_circuit().unwrap();
        let expected = Fr::from_str(
            "17396252260025783793058854431926620863655419045074533465745990270806947938816"
        );
        assert_eq!(circuit.get_public_inputs().unwrap()[0],expected.unwrap());
    }

    #[test]
    fn test_prove_verify() {
        let mut rng = StdRng::seed_from_u64(0);
        let c = build_circuit().unwrap();
        let (pk, pvk) = setup().unwrap();
        let (proof, pubs) = prove(&pk, c, &mut rng).unwrap();

        // 1. 正しい proof は通る
        assert!(verify(&pvk, &pubs, &proof).unwrap());

        // 2. public input（root）を1つ変えると落ちる
        let mut bad = pubs.clone();
        bad[0] += Fr::from(1u64);
        assert!(!verify(&pvk, &bad, &proof).unwrap());
    }

    #[test]
    fn test_rln_secret_recovery() {
        // 小さい木を1個作って、1人分の secret/salt と proof を用意
        let secret = Fr::from_str("103").unwrap();
        let salt = Fr::from_str("9003").unwrap();
        let leaf = crate::merkle::hash_leaf(secret, salt);
        let tree = crate::merkle::MerkleTree::from_leaves(vec![leaf; 5], 4);
        let p = tree.proof(0);
        let siblings: Vec<Fr> = p.iter().map(|x| x.0).collect();
        let path_indices: Vec<bool> = p.iter().map(|x| x.1).collect();

        let epoch = Fr::from(1u64);
        let challenge1 = Fr::from(111u64);
        let challenge2 = Fr::from(222u64);

        let c1 = build_circuit_with_inputs(secret, salt, epoch, challenge1, &path_indices, &siblings).unwrap();
        let pubs1 = c1.get_public_inputs().unwrap(); // [root, y, nullifier, epoch, challenge]

        let c2 = build_circuit_with_inputs(secret, salt, epoch, challenge2, &path_indices, &siblings).unwrap();
        let pubs2 = c2.get_public_inputs().unwrap();

        // 同一人物・同一epochなら nullifier は一致するはず
        assert_eq!(pubs1[2], pubs2[2]);

        let x1 = crate::merkle::hash_single(challenge1);
        let x2 = crate::merkle::hash_single(challenge2);

        let recovered = recover_secret(x1, pubs1[1], x2, pubs2[1]);
        assert_eq!(recovered, secret);
    }
}
