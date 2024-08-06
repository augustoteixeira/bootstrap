use enterpolation::Generator;
use image::{Rgb, RgbImage};
use std::path;

use super::memory::Memory;
use super::u64_vec::U64Array;

use enterpolation::linear::Linear;
use palette::LinSrgb;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn write_color_image(grid: &U64Array, path: String) {
    let jets = [
        LinSrgb::new(0.00, 0.00, 0.50),
        LinSrgb::new(0.00, 0.00, 1.00),
        LinSrgb::new(0.00, 0.50, 1.00),
        LinSrgb::new(0.00, 1.00, 1.00),
        LinSrgb::new(0.50, 1.00, 0.50),
        LinSrgb::new(1.00, 1.00, 0.00),
        LinSrgb::new(1.00, 0.50, 0.00),
        LinSrgb::new(1.00, 0.00, 0.00),
        LinSrgb::new(0.50, 0.00, 0.00),
    ];
    let spectral = [
        LinSrgb::new(0.00, 0.00, 0.00),
        LinSrgb::new(0.50, 0.00, 0.50),
        LinSrgb::new(0.00, 0.00, 1.00),
        LinSrgb::new(0.00, 0.50, 1.00),
        LinSrgb::new(0.00, 1.00, 1.00),
        LinSrgb::new(0.50, 1.00, 0.50),
        LinSrgb::new(1.00, 1.00, 0.00),
        LinSrgb::new(1.00, 0.50, 0.00),
        LinSrgb::new(1.00, 0.00, 0.00),
        LinSrgb::new(1.00, 0.50, 0.50),
        LinSrgb::new(1.00, 1.00, 1.00),
    ];

    // setup normalization and interpolation and color palette
    let min_value: f64 = *grid.data.iter().min().unwrap() as f64;
    let max_value: f64 = *grid.data.iter().max().unwrap() as f64;
    let gradient = Linear::builder()
        .elements(spectral)
        .equidistant::<f64>()
        .domain(min_value, max_value + 1.0)
        .build()
        .unwrap();

    let side: u32 = grid.get_side().try_into().unwrap();
    let mut img = RgbImage::new(side, side);
    for x in 0..side {
        for y in 0..side {
            if grid.get(x as i32, y as i32) {
                let value = grid.get_value(x as i32, y as i32) as f64;
                let rgb = gradient.gen(value).into_format::<u8>();
                let color = Rgb([rgb.red, rgb.green, rgb.blue]);
                img.put_pixel(x, side - y - 1, color);
            } else {
                img.put_pixel(x, side - y - 1, Rgb([0, 0, 0]));
            }
        }
    }
    img.save_with_format(&path::Path::new(&path), image::ImageFormat::Png)
        .unwrap();
}

#[allow(dead_code)]
pub fn print_image(grid: &impl Memory) {
    let side: i32 = grid.get_side().try_into().unwrap();
    for y in (0..side).rev() {
        print!("|");
        for x in 0..side {
            print!(
                "{:}",
                match grid.get(x, y) {
                    true => "x",
                    false => ".",
                }
            );
        }
        println!("|");
    }
}

#[allow(dead_code)]
pub fn pedantic_print(grid: &impl Memory) {
    let side = grid.get_side();
    for y in 0..side {
        for x in 0..side {
            println!("x = {:}, y = {:}, v = {:}", x, y, grid.get(x, y));
        }
    }
}
