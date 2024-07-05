use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::time::Instant;

use super::aux::write_image;
use super::bool_vec::ByteArray;
use super::frobose::{frobose_droplet_size, frobose_run};
use super::memory::Memory;
use super::modified::{modified_droplet_size, modified_run};
use super::Model;
use super::{Batch, Single};

pub fn fill_random(grid: &mut impl Memory, p: f64, seed: u64) {
    // TODO: Check for other RNG with faster speeds, chacha is crypto-secure
    // TODO: Read about thread-safe RNG
    // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let mut rng = SmallRng::seed_from_u64(seed);
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

const SIZE_SUBBATCH: usize = 100;

pub fn process_batch(
    p: f64,
    seed_offset: u64,
    number_filled_required: usize,
    model: Model,
) -> Batch {
    let start = Instant::now();
    let side: i32 = match model {
        Model::Modified => modified_droplet_size(p).try_into().unwrap(),
        Model::Frobose => frobose_droplet_size(p).try_into().unwrap(),
    };
    let bar = ProgressBar::new(number_filled_required as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:30.cyan/blue} {pos:>7}/{len:7} {msg} ETA {eta}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    let mut number_filled = 0;
    let mut number_samples = 0;
    'outer: loop {
        bar.set_position(number_filled);
        bar.set_message(format!("Filled {:}", number_filled));
        let mut mask: [bool; SIZE_SUBBATCH] = [false; SIZE_SUBBATCH];
        mask.par_iter_mut().enumerate().for_each(|(j, has_filled)| {
            let mut grid = ByteArray::new(side);
            fill_random(
                &mut grid,
                p,
                seed_offset + (number_samples as u64) + j as u64,
            );
            let _final_step = modified_run(&mut grid);
            if is_filled(&grid) {
                *has_filled = true;
            }
        });
        for j in 0..SIZE_SUBBATCH {
            if mask[j] {
                number_filled += 1;
            }
            if number_filled == (number_filled_required as u64) {
                break 'outer;
            }
            number_samples += 1;
        }
    }
    bar.finish_and_clear();
    let proportion_filled = (number_filled as f64) / (number_samples as f64);
    let duration = start.elapsed();
    Batch {
        infection_probability: p,
        side: side.try_into().unwrap(),
        seed_offset,
        number_filled: number_filled as usize,
        num_samples: number_samples,
        proportion_filled,
        time_ellapsed: duration,
    }
}

pub fn process_single(
    p: f64,
    side: Option<u64>,
    seed_offset: u64,
    file_path: String,
    should_write: bool,
    model: Model,
) -> Single {
    let side: u64 = match side {
        Some(s) => s,
        None => modified_droplet_size(p) as u64,
    };
    let start = Instant::now();
    let mut grid = ByteArray::new(side as i32);
    fill_random(&mut grid, p, seed_offset as u64);
    let final_step = match model {
        Model::Modified => modified_run(&mut grid),
        Model::Frobose => frobose_run(&mut grid),
    };
    let duration = start.elapsed();
    if should_write {
        write_image(&grid, file_path.to_string());
    }
    Single {
        infection_probability: p,
        side: side.try_into().unwrap(),
        seed_offset,
        was_filled: is_filled(&grid),
        time_ellapsed: duration,
        final_step,
    }
}
