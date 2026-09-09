use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomCircuit, CircomConfig, CircomBuilder};
use ark_groth16::{Groth16, Proof, ProvingKey, PreparedVerifyingKey};
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use std::str::FromStr ;

pub fn build_circuit() -> color_eyre::Result<CircomCircuit<Fr>>{
    let cfg = CircomConfig::<Fr>::new(
        "circuits/main_js/main.wasm",
        "circuits/main.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.load_input_json("circuits/input.json")?;  
    Ok(builder.build()?)
}


pub fn setup(circuit: CircomCircuit<Fr>, rng: &mut StdRng)
    -> color_eyre::Result<(ProvingKey<Bn254>, PreparedVerifyingKey<Bn254>)>{
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng)?;
    let prepared_verifying_key = Groth16::<Bn254>::process_vk(&vk)?;
    color_eyre::Result::Ok((pk, prepared_verifying_key))
    }

pub fn prove(pk: &ProvingKey<Bn254>, circuit: CircomCircuit<Fr>, rng: &mut StdRng)
    -> color_eyre::Result<(Proof<Bn254>, Vec<Fr>)> {
    // circuit は prove で消費されるので、public inputs を先に取る
    let public_inputs = circuit.get_public_inputs().unwrap();
    let proof = Groth16::<Bn254>::prove(pk, circuit, rng)?;
    Ok((proof, public_inputs))
}

pub fn verify(pvk: &PreparedVerifyingKey<Bn254>, public_inputs: &[Fr], proof: &Proof<Bn254>)
    -> color_eyre::Result<bool> {
    Ok(Groth16::<Bn254>::verify_with_processed_vk(pvk, public_inputs, proof)?)
}

// pub fn build_witness() -> color_eyre::Result<Vec<Fr>> {
    
// }

#[cfg(test)]
mod test{
    use super::*;

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
        let (pk, pvk) = setup(c.clone(), &mut rng).unwrap();
        let (proof, pubs) = prove(&pk, c, &mut rng).unwrap();

        // 1. 正しい proof は通る
        assert!(verify(&pvk, &pubs, &proof).unwrap());

        // 2. public input（root）を1つ変えると落ちる
        let mut bad = pubs.clone();
        bad[0] += Fr::from(1u64);
        assert!(!verify(&pvk, &bad, &proof).unwrap());
    }
}
