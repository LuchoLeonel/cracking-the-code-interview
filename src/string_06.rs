use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;

fn rotate_matrix() {
    let current_dir = env::current_dir().unwrap();
    let path = format!("{}/public/image.bmp", current_dir.display());
    let mut file = File::open(&path).expect("to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("to read buffer");

    let file_type = &buffer[0..2];
    if file_type != b"BM" {
        panic!("El archivo no es un BMP válido.");
    }

    let width = u32::from_le_bytes([buffer[18], buffer[19], buffer[20], buffer[21]]) as usize;
    let height = u32::from_le_bytes([buffer[22], buffer[23], buffer[24], buffer[25]]) as usize;
    let bits_per_pixel = u16::from_le_bytes([buffer[28], buffer[29]]);

    if bits_per_pixel != 24 {
        panic!("Actualmente solo se soportan BMPs de 24 bits por píxel.");
    }

    let row_size = ((bits_per_pixel as usize * width + 31) / 32) * 4;

    let mut pixels = vec![vec![[0u8; 3]; width]; height];

    let offset = u32::from_le_bytes([buffer[10], buffer[11], buffer[12], buffer[13]]) as usize;
    for y in 0..height {
        for x in 0..width {
            let pixel_start = offset + y * row_size + x * 3;
            let pixel = &buffer[pixel_start..pixel_start + 3];
            pixels[y][x] = [pixel[2], pixel[1], pixel[0]];
        }
    }

    let mut rotated_pixels = vec![vec![[0u8; 3]; height]; width];
    for y in 0..height {
        for x in 0..width {
            rotated_pixels[x][height - y - 1] = pixels[y][x];
        }
    }

    let mut new_image_data = vec![0u8; row_size * width];
    for y in 0..width {
        for x in 0..height {
            let pixel_start = y * row_size + x * 3;
            let pixel = rotated_pixels[y][x];
            new_image_data[pixel_start..pixel_start + 3].copy_from_slice(&[pixel[2], pixel[1], pixel[0]]);
        }
    }

    let new_buffer = [&buffer[..offset], &new_image_data].concat();

    let new_path = format!("{}/public/image_90_degrees.bmp", current_dir.display());
    let mut new_file = File::create(&new_path).expect("to create file");
    new_file.write_all(&new_buffer).expect("to write buffer");
}

pub fn run() {
    rotate_matrix();
}
