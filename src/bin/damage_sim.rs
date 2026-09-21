use rand::RngExt;
use std::collections::HashMap;

fn main() {
    let num_member = 15.0;
    let attack_rate = 0.33;
    let comp_rate = 0.015;
    let avg_damage = 7_850_000.0;

    let receiver = num_member * attack_rate;

    let victim_noservice = receiver * comp_rate;
    let total_damage_noservice = victim_noservice * avg_damage;

    let block_rate = 0.95;
    let victim_inservice = receiver * comp_rate * (1.0 - block_rate);
    let total_damage_inservice = victim_inservice * avg_damage;

    println!("{{in_service: {{victim: {:}, total_damage:{:}}}}}", victim_inservice, total_damage_inservice);
    println!("{{no_service: {{victim: {:}, total_damage:{:}}}}}", victim_noservice, total_damage_noservice);
}