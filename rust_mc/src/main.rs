mod memory;
use clap::Parser;
//use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

mod aux;
mod bool_vec;
mod modified;
//use aux::write_image;
mod operations;
use operations::process_batch;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(short, long, value_name = "INFECTION_PROBABILITY")]
    // p: f64,
    #[arg(short, long, value_name = "MAX_M")]
    max_m: u64,
    #[arg(short, long, value_name = "SEED_OFFSET")]
    offset: u64,
    #[arg(short, long, value_name = "SAMPLE_MULTIPLIER")]
    sample_multiplier: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Batch {
    infection_probability: f64,
    side: usize,
    num_samples: usize,
    seed_offset: u64,
    number_filled: usize,
    proportion_filled: f64,
    time_ellapsed: Duration,
}

//fn make_image() {
//        write_image(&grid, "test1.png".to_string());
//}

fn main() {
    let cli = Cli::parse();
    let max_m = cli.max_m;
    let sample_multiplier = cli.sample_multiplier;
    for m in 0..max_m {
        println! {"Starting batch with m = {:}", m};
        let p = (0.5 as f64).powf(2.0 + (m as f64) * 0.2);
        let batch = process_batch(p, cli.offset, sample_multiplier);
        println!("{:#?}", batch);
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::bool_vec::ByteArray;
    use super::memory::Memory;
    use super::modified::modified_step;

    #[test]
    fn basic_set_get() {
        let mut grid = ByteArray::new(4);
        assert_eq!(grid.get(0, 0), false);
        assert_eq!(grid.get(-111, -12), false);
        grid.set(0, 0);
        assert_eq!(grid.get(0, 0), true);
    }

    #[test]
    fn basic_modified_update() {
        let mut grid = ByteArray::new(4);
        grid.set(0, 0);
        grid.set(1, 1);
        modified_step(&mut grid);
        assert_eq!(grid.get(1, 0), true);
        assert_eq!(grid.get(0, 1), true);
    }

    #[test]
    fn basic_modified_stable() {
        let mut grid = ByteArray::new(4);
        grid.set(0, 0);
        grid.set(1, 0);
        modified_step(&mut grid);
        assert_eq!(grid.get(1, 1), false);
        assert_eq!(grid.get(0, 1), false);
    }

    #[test]
    fn basic_modified_stable2() {
        let mut grid = ByteArray::new(4);
        grid.set(1, 0);
        grid.set(1, 1);
        modified_step(&mut grid);
        assert_eq!(grid.get(0, 0), false);
        assert_eq!(grid.get(0, 1), false);
    }
}
