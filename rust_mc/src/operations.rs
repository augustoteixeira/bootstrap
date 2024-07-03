use rand::distributions::{Bernoulli, Distribution};
use rand::SeedableRng;

use super::memory::Memory;

pub fn fill_random(grid: &mut impl Memory, p: f64, seed: u64) {
    // TODO: Check for other RNG with faster speeds, chacha is crypto-secure
    // TODO: Read about thread-safe RNG
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let bernoulli = Bernoulli::new(p).unwrap();
    let side = grid.get_side();
    for x in 0..side {
        for y in 0..side {
            let v = bernoulli.sample(&mut rng);
            if v {
                grid.set(x, y)
            }
        }
    }
}

#[allow(dead_code)]
pub fn is_filled(grid: &impl Memory) -> bool {
    let side = grid.get_side();
    for y in 0..side {
        for x in 0..side {
            if !grid.get(x, y) {
                return false;
            }
        }
    }
    return true;
}
