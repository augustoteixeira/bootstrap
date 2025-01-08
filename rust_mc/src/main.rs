mod memory;
use clap::Parser;
use frobose::frobose_droplet_size;
use modified::modified_droplet_size;
//use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

mod aux;
mod bool_vec;
mod frobose;
mod modified;
mod operations;
mod u64_vec;
use operations::{process_bar, process_batch, process_droplet, process_single};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Model {
    Modified,
    Frobose,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "SEED_OFFSET")]
    offset: u64,
    #[arg(short, long, value_name = "MODEL")]
    model: Model,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[derive(Debug, Clone)]
enum Command {
    Batch {
        #[arg(long, value_name = "MAX_M")]
        max_m: u64,
        #[arg(long, value_name = "MIN_M")]
        min_m: u64,
        #[arg(short, long, value_name = "NUMBER_OF_FILLED_BOXES_REQUIRED")]
        number_filled_required: usize,
    },
    Single {
        #[arg(short, value_name = "INFECTION_PROBABILITY")]
        p: f64,
        #[arg(short, value_name = "SIDE_LENGTH")]
        side: Option<u64>,
        #[arg(short, value_name = "SHOULD_WRITE_IMAGE")]
        write: bool,
    },
    Droplet {
        #[arg(short, value_name = "INFECTION_PROBABILITY")]
        p: f64,
        #[arg(short, value_name = "SHOULD_WRITE_IMAGE")]
        write: bool,
    },
    ColorBar {
        #[arg(short, value_name = "HEIGHT")]
        size: u64,
        #[arg(short, value_name = "SHOULD_WRITE_IMAGE")]
        write: bool,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
struct Batch {
    infection_probability: f64,
    side: usize,
    num_samples: usize,
    seed_offset: u64,
    number_filled: usize,
    proportion_filled: f64,
    time_ellapsed: Duration,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Single {
    infection_probability: f64,
    side: usize,
    seed_offset: u64,
    was_filled: bool,
    time_ellapsed: Duration,
    final_step: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Droplet {
    infection_probability: f64,
    side: usize,
    seed_offset: u64,
    time_ellapsed: Duration,
    final_step: usize,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Command::ColorBar { size, write } => {
            process_bar(size as u64, "bar.png".to_string(), write);
        }
        Command::Batch {
            max_m,
            min_m,
            number_filled_required,
        } => {
            let max_m = max_m;
            for m in min_m..=max_m {
                println! {"Starting batch with m = {:}", m};
                let p = (0.5 as f64).powf(2.0 + (m as f64) * 0.2);
                let batch = process_batch(
                    p,
                    cli.offset,
                    number_filled_required,
                    cli.model.clone(),
                );
                println!("{:#?}", batch);
                println!("");
            }
        }
        Command::Single { p, side, write } => {
            let side: usize = match side {
                Some(s) => s as usize,
                None => match cli.model {
                    Model::Modified => modified_droplet_size(p),
                    Model::Frobose => frobose_droplet_size(p),
                },
            };
            let single = process_single(
                p,
                side as u64,
                cli.offset,
                "test.png".to_string(),
                write,
                cli.model,
            );
            println!("{:#?}", single);
        }
        Command::Droplet { p, write } => {
            let side: usize = match cli.model {
                Model::Modified => modified_droplet_size(p),
                Model::Frobose => frobose_droplet_size(p),
            };
            let droplet = process_droplet(
                p,
                side,
                cli.offset,
                "test.png".to_string(),
                write,
                cli.model,
            );
            println!("{:#?}", droplet);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bool_vec::ByteArray;
    use super::memory::Memory;
    use super::modified::modified_step;

    #[test]
    fn basic_set_get() {
        let mut grid = ByteArray::new_filled_with_false(4);
        assert_eq!(grid.get(0, 0), false);
        assert_eq!(grid.get(-111, -12), false);
        grid.set(0, 0);
        assert_eq!(grid.get(0, 0), true);
    }

    #[test]
    fn basic_modified_update() {
        let mut grid = ByteArray::new_filled_with_false(4);
        grid.set(0, 0);
        grid.set(1, 1);
        modified_step(&mut grid);
        assert_eq!(grid.get(1, 0), true);
        assert_eq!(grid.get(0, 1), true);
    }

    #[test]
    fn basic_modified_stable() {
        let mut grid = ByteArray::new_filled_with_false(4);
        grid.set(0, 0);
        grid.set(1, 0);
        modified_step(&mut grid);
        assert_eq!(grid.get(1, 1), false);
        assert_eq!(grid.get(0, 1), false);
    }

    #[test]
    fn basic_modified_stable2() {
        let mut grid = ByteArray::new_filled_with_false(4);
        grid.set(1, 0);
        grid.set(1, 1);
        modified_step(&mut grid);
        assert_eq!(grid.get(0, 0), false);
        assert_eq!(grid.get(0, 1), false);
    }
}
