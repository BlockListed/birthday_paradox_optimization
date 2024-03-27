use std::collections::HashSet;

use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

const CARLO_SIM_COUNT: u64 = 1_000_000;

fn main() {
    let duplicate_bdays = (0..CARLO_SIM_COUNT).map(carlo_sim).filter(|e| *e).count();

    println!("{duplicate_bdays} out of {CARLO_SIM_COUNT} had duplicate bdays!");
}

fn rand_generator(seed: u64) -> impl RngCore {
    rand_chacha::ChaChaRng::seed_from_u64(seed)
}

fn carlo_sim(idx: u64) -> bool {
    let mut rng = rand_generator(idx);

    let bdays: Vec<u16> = (0..23).map(|_| rng.gen_range(1..366)).collect();

    let duplicate_bday = {
        let mut set = HashSet::with_capacity(bdays.len());

        bdays.into_iter().fold(false, |acc, e| acc || !set.insert(e))
    };

    duplicate_bday
}
