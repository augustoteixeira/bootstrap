use rand::distributions::{Bernoulli, Distribution};
use rand::SeedableRng;

mod memory;
use memory::Memory;

mod bool_vec;
use bool_vec::ByteArray;

mod aux;
use aux::{print_image, write_image};

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
    for x in 0..(side - 1) {
        for y in 0..(side - 1) {
            (nw, ne, sw, se) = (
                grid.get(x, y + 1),
                grid.get(x + 1, y + 1),
                grid.get(x, y),
                grid.get(x + 1, y),
            );
            if nw && se {
                if updated == false {
                    if !(nw && ne && sw && se) {
                        updated = true;
                    }
                }
                grid.set(x, y);
                grid.set(x + 1, y + 1);
            }
            if ne && sw {
                if updated == false {
                    if !(nw && ne && sw && se) {
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

fn main() {
    let side = 200;
    let p = 0.06;
    let iterations = 10000;
    let mut grid = ByteArray::new(side);
    let seed = 234234;
    fill(&mut grid, p, seed);
    if side <= 80 {
        print_image(&grid);
    }
    if side <= 1000 {
        write_image(&grid, "test0.png".to_string());
    }
    for _ in 0..iterations {
        if !modified_step(&mut grid) {
            break;
        }
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
}
