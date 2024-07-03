use image::{Rgb, RgbImage};
use std::path;

use super::memory::Memory;

pub fn write_image(grid: &impl Memory, path: String) {
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

pub fn print_image(grid: &impl Memory) {
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
