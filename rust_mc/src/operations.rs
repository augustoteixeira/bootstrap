use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Bernoulli, Distribution};
use rand::SeedableRng;
use std::time::Instant;

use super::modified::{droplet_size, modified_run, number_of_samples};

use super::bool_vec::ByteArray;
use super::memory::Memory;
use super::Batch;

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

pub fn process_batch(
    p: f64,
    seed_offset: u64,
    sample_multiplier: usize,
) -> Batch {
    let start = Instant::now();
    let side: i32 = droplet_size(p).try_into().unwrap();
    let num_samples = sample_multiplier * number_of_samples(p);
    let bar = ProgressBar::new(num_samples as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:30.cyan/blue} {pos:>7}/{len:7} {msg} ETA {eta}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    let mut number_filled = 0;
    for seed in 0..num_samples {
        bar.inc(1);
        bar.set_message(format!("Filled {:}", number_filled));
        let mut grid = ByteArray::new(side);
        fill_random(&mut grid, p, seed_offset + seed as u64);
        let _final_step = modified_run(&mut grid);
        if is_filled(&grid) {
            number_filled += 1;
        }
    }
    bar.finish_and_clear();
    let proportion_filled = (number_filled as f64) / (num_samples as f64);
    let duration = start.elapsed();
    Batch {
        infection_probability: p,
        side: side.try_into().unwrap(),
        num_samples,
        seed_offset,
        number_filled,
        proportion_filled,
        time_ellapsed: duration,
    }
}
