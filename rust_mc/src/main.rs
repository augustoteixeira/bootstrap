use image::{Rgb, RgbImage};
use rand::distributions::{Bernoulli, Distribution};
use rand::SeedableRng;
use std::path;

type Index = i32;

trait Memory {
    fn new(side: Index) -> Self;
    fn get_side(&self) -> Index;
    fn get(&self, x: Index, y: Index) -> bool;
    fn set(&mut self, x: Index, y: Index, v: bool);
}

struct ByteArray {
    side: Index,
    data: Vec<bool>,
}

impl Memory for ByteArray {
    fn new(side: Index) -> Self {
        if side <= 0 {
            panic!("Size non-positive")
        }
        ByteArray {
            side,
            data: vec![false; (side * side).try_into().unwrap()],
        }
    }

    fn get_side(&self) -> Index {
        self.side
    }

    fn get(&self, x: Index, y: Index) -> bool {
        if x < 0 || x >= self.side {
            return false;
        }
        if y < 0 || x >= self.side {
            return false;
        }
        let index: usize =
            (x + y * self.side).try_into().expect("Index out of bounds");
        return unsafe { *self.data.get_unchecked(index) };
    }
    fn set(&mut self, x: Index, y: Index, v: bool) {
        if x < 0 || x >= self.side {
            return;
        }
        if y < 0 || x >= self.side {
            return;
        }
        let index: usize =
            (x + y * self.side).try_into().expect("Index out of bounds");
        self.data[index] = v;
    }
}

fn fill(grid: &mut impl Memory, p: f64, seed: u64) {
    // TODO: Check for other RNG with faster speeds, chacha is crypto-secure
    // TODO: Read about thread-safe RNG
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let bernoulli = Bernoulli::new(p).unwrap();
    let side = grid.get_side();
    for x in 0..side {
        for y in 0..side {
            let v = bernoulli.sample(&mut rng);
            grid.set(x, y, v)
        }
    }
    grid.set(0, 0, true);
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
                grid.set(x, y, true);
                grid.set(x + 1, y + 1, true);
            }
            if ne && sw {
                if updated == false {
                    if !(nw && ne && sw && se) {
                        updated = true;
                    }
                }
                grid.set(x + 1, y, true);
                grid.set(x, y + 1, true);
            }
        }
    }
    return updated;
}

fn write_image(grid: &impl Memory, path: String) {
    let side: u32 = grid.get_side().try_into().unwrap();
    let mut img = RgbImage::new(side, side);
    for x in 0..side {
        for y in 0..side {
            if grid.get(x as i32, y as i32) {
                img.put_pixel(x, side - y - 1, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x, side - y - 1, Rgb([0, 0, 0]));
            }
        }
    }
    img.save_with_format(&path::Path::new(&path), image::ImageFormat::Png)
        .unwrap();
}

fn print_image(grid: &impl Memory) {
    let side: i32 = grid.get_side().try_into().unwrap();
    for y in 0..side {
        for x in 0..side {
            print!(
                "{:}",
                match grid.get(x, side - y - 1) {
                    true => "x",
                    false => " ",
                }
            );
        }
        println!("");
    }
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
        grid.set(0, 0, true);
        assert_eq!(grid.get(0, 0), true);
    }
}
