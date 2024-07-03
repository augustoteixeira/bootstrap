mod memory;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use memory::Memory;

mod bool_vec;
use bool_vec::ByteArray;

mod aux;
use aux::write_image;

mod operations;
use operations::{fill_random, is_filled};

mod modified;
use modified::modified_run;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "SIDE_LENGTH")]
    side: i32,
    #[arg(short, long, value_name = "PROBABILITY")]
    p: f64,
    #[arg(short, long, value_name = "NUMBER_SAMPLES")]
    num_samples: usize,
    #[arg(short, long, value_name = "SEED_OFFSET")]
    offset: u64,
}

fn main() {
    let cli = Cli::parse();
    let side = cli.side;
    let p = cli.p;
    let num_samples = cli.num_samples;
    let seed_offset = cli.offset;
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
        //if side <= 1000 {
        //    write_image(&grid, "test0.png".to_string());
        //}
        //print!("sample = {:10}. ", seed);
        let _final_step = modified_run(&mut grid);
        if is_filled(&grid) {
            number_filled += 1;
            //print!("It filled!  ");
        } else {
            //print!("Not filled! ");
        }
        //write_image(&grid, "test.png".to_string());
        //println!("Final step = {:}", final_step);
        //if side <= 80 {
        //    print_image(&grid);
        //}
        //if side <= 1000 {
        //    write_image(&grid, "test1.png".to_string());
        //}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
