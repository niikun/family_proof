use ark_bn254::{Bn254, Fr};
use ark_circom::{CircomBuilder, CircomConfig};
use std::str::FromStr;

pub fn build_witness() -> color_eyre::Result<Vec<Fr>> {
    let cfg = CircomConfig::<Fr>::new(
        "circuits/main_js/main.wasm",
        "circuits/main.r1cs",
    )?;
    let mut builder = CircomBuilder::new(cfg);
    builder.load_input_json("circuits/input.json")?;   // leaf / pathIndices / siblings をまとめて読む

    let circom = builder.build()?;                     // ここで witness 計算
    let public_inputs = circom.get_public_inputs().unwrap(); // Vec<Fr>、先頭が root
    Ok(public_inputs)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_build_witness(){
        let pubs = build_witness().unwrap();
        let expected = Fr::from_str(
            "17396252260025783793058854431926620863655419045074533465745990270806947938816"
        ).unwrap();
        assert_eq!(pubs[0],expected);
    }
}