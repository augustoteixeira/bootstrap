use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::time::Instant;

use super::aux::{write_color_image, write_image};
use super::bool_vec::ByteArray;
use super::frobose::{frobose_droplet_size, frobose_run, frobose_step};
use super::memory::Memory;
use super::modified::{modified_droplet_size, modified_run, modified_step};
use super::u64_vec::U64Array;
use super::Model;
use super::{Batch, Droplet, Single};

pub fn clear_grid(grid: &mut U64Array) {
    let side = grid.get_side() as usize;
    grid.set_next_value(0);
    for x in 0..side {
        for y in 0..side {
            grid.data[y * side + x] = u64::MAX;
        }
    }
}

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
            let mut grid = ByteArray::new_filled_with_false(side);
            fill_random(
                &mut grid,
                p,
                seed_offset + (number_samples as u64) + j as u64,
            );
            match model {
                Model::Modified => _ = modified_run(&mut grid),
                Model::Frobose => _ = frobose_run(&mut grid),
            }
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
    side: u64,
    seed_offset: u64,
    file_path: String,
    should_write: bool,
    model: Model,
) -> Single {
    let start = Instant::now();
    let mut grid = ByteArray::new_filled_with_false(side as i32);
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

pub fn process_bar(side: u64, file_path: String, should_write: bool) {
    let mut grid = U64Array::new_filled_with_false(side as i32);
    for y in 0..(side as i32) {
        grid.set_next_value(y as u64);
        for x in 0..(side as i32) {
            grid.set(x, y);
        }
    }
    if should_write {
        write_color_image(&grid, file_path.to_string());
    }
}

pub const INITIAL_INFECTION_VALUE: u64 = u64::MAX - 1;

pub fn process_droplet(
    p: f64,
    side: usize,
    seed_offset: u64,
    file_path: String,
    should_write: bool,
    model: Model,
) -> Droplet {
    let start = Instant::now();
    let mut grid = U64Array::new_filled_with_false(side as i32);
    let mut seed_increment = 0;
    let mut duration;
    let mut final_step;
    println!("Trying to find a droplet...");
    loop {
        grid.set_next_value(INITIAL_INFECTION_VALUE);
        fill_random(&mut grid, p, (seed_offset + seed_increment) as u64);
        grid.set_next_value(0);
        seed_increment += 1;
        final_step = film_evolution(model.clone(), &mut grid);
        duration = start.elapsed();
        if is_filled(&grid) {
            break;
        };
        print!(
            "{}\rp = {p}, final_step = {final_step}, duration = {:?}, side = {side}         ",
            8u8 as char, duration
        );
        clear_grid(&mut grid);
    }
    if should_write {
        write_color_image(&grid, file_path.to_string());
    }
    Droplet {
        infection_probability: p,
        side: side.try_into().unwrap(),
        seed_offset,
        time_ellapsed: duration,
        final_step: final_step as usize,
    }
}

pub fn film_evolution(model: Model, grid: &mut U64Array) -> i32 {
    let mut i = 0;
    grid.set_next_value(i);
    let mut updated;
    loop {
        updated = match model {
            Model::Modified => modified_step(grid),
            Model::Frobose => frobose_step(grid),
        };
        if !updated {
            break;
        }
        i += 1;
        grid.set_next_value(i);
    }
    return i.try_into().unwrap();
}
