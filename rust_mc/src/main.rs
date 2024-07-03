use rand::distributions::{Bernoulli, Distribution};
use rand::SeedableRng;

mod memory;
use memory::Memory;

mod bool_vec;
use bool_vec::ByteArray;

mod aux;
use aux::{pedantic_print, print_image, write_image};

fn fill(grid: &mut impl Memory, p: f64, seed: u64) {
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

fn modified_step(grid: &mut impl Memory) -> bool {
    // we think of the origin as being in the left bottom corner
    let mut nw: bool;
    let mut ne: bool;
    let mut sw: bool;
    let mut se: bool;
    let side = grid.get_side();
    let mut updated = false;
    for y in 0..(side - 1) {
        for x in 0..(side - 1) {
            (nw, ne, sw, se) = (
                grid.get(x, y + 1),
                grid.get(x + 1, y + 1),
                grid.get(x, y),
                grid.get(x + 1, y),
            );
            //println!("{:},{:},{:},{:},{:},{:}", x, y, nw, ne, sw, se);
            if nw && se {
                if updated == false {
                    if !(ne && sw) {
                        updated = true;
                    }
                }
                grid.set(x, y);
                grid.set(x + 1, y + 1);
            }
            if ne && sw {
                if updated == false {
                    if !(nw && se) {
                        updated = true;
                    }
                }
                grid.set(x + 1, y);
                grid.set(x, y + 1);
            }
        }
    }
    return updated;
}

fn modified_run(grid: &mut impl Memory) {
    let mut i = 0;
    let mut updated;
    loop {
        println!("{:}", i);
        //print_image(grid);
        //pedantic_print(grid);
        updated = modified_step(grid);
        i += 1;
        if !updated {
            break;
        }
    }
}

fn is_filled(grid: &impl Memory) -> bool {
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

fn main() {
    let side = 200;
    let p = 0.1;
    let mut grid = ByteArray::new(side);
    let seed = 1123;
    fill(&mut grid, p, seed);
    if side <= 1000 {
        write_image(&grid, "test0.png".to_string());
    }
    modified_run(&mut grid);
    if side <= 80 {
        //print_image(&grid);
    }
    if is_filled(&grid) {
        println!("It filled!");
    }
    if side <= 1000 {
        write_image(&grid, "test1.png".to_string());
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
