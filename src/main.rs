use bitvec::bitvec;

use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

const CARLO_SIM_COUNT: u64 = 1_000_000;

fn main() {
    let duplicate_bdays = (0..CARLO_SIM_COUNT).into_par_iter().map(carlo_sim).filter(|e| *e).count();

    println!("{duplicate_bdays} out of {CARLO_SIM_COUNT} had duplicate bdays!");
}

fn rand_generator(seed: u64) -> impl RngCore {
    rand_chacha::ChaChaRng::seed_from_u64(seed)
}

fn carlo_sim(idx: u64) -> bool {
    let mut rng = rand_generator(idx);

    let bdays: Vec<u16> = (0..23).map(|_| rng.gen_range(1..366)).collect();

    let duplicate_bday = {
        let mut set = bitvec![0; 365];

        bdays.into_iter().fold(false, |acc, e| acc || set.replace(e as usize - 1, true))
    };

    duplicate_bday
}
