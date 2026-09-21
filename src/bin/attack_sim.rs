use rand::RngExt;
use ark_bn254::Fr;
use std::str::FromStr;
use family_proof::merkle;
use family_proof::proof;

fn main() {
    let mut rng = rand::rng();
    let horizon_epochs = 10;
    let secret = Fr::from_str("103").unwrap(); 
    let num_member = 15.0;
    let attack_rate = 0.33;
    let comp_rate = 0.015;
    let avg_damage = 7_850_000.0;
    let mut damage = 0.0;
    let mut caught_at: Option<i32> = None;
    for epoch_idx in 0..horizon_epochs {
        let epoch = Fr::from(epoch_idx as u64);
        let challenge1_raw: u64 = rng.random();
        let challenge2_raw: u64 = rng.random();
        let challenge1 = Fr::from_str(&challenge1_raw.to_string()).unwrap();
        let challenge2 = Fr::from_str(&challenge2_raw.to_string()).unwrap();
        let n_attempts = rng.random_range(1..10);
        if n_attempts >= 2 {
            let (x1, y1) = rln_share(secret, epoch, challenge1);
            let (x2, y2) = rln_share(secret, epoch, challenge2); // 異なるchallenge
            let recovered = proof::recover_secret(x1, y1, x2, y2);
            assert_eq!(recovered, secret); // 検知が実際に成立することを実証
            caught_at = Some(epoch_idx);
            break; // 失効・攻撃終了
        }else if n_attempts == 1 {
            damage += avg_damage * comp_rate; 
        }
    }
    println!("caught_at: {:?}, damage: {}", caught_at, damage);
}

fn rln_share(secret: Fr, epoch: Fr, challenge: Fr) -> (Fr, Fr) {
    let a1 = merkle::hash_pair(&secret, &epoch);
    let x = merkle::hash_single(challenge);
    let y = secret + a1 * x;
    (x, y)
}

