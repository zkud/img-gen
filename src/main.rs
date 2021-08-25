use jpeg_encoder::{Encoder, ColorType};

const RECURSION_DEPTH: u8 = 50;
const IMAGE_WIDTH: usize = 2048;
const IMAGE_HEIGHT: usize = 1024;
const IMAGE_SIZE: usize = 6291456;

fn main() {
    let mut image: [u8; IMAGE_SIZE] = [0; IMAGE_SIZE];

    draw_mask(&mut image);

    fill_with_colors(&mut image);

    output_image(&mut image);
}

#[inline]
fn draw_mask(image: &mut [u8]) {
}

#[inline]
fn fill_with_colors(image: &mut [u8]) {
    for pixel_index in 0..IMAGE_HEIGHT*IMAGE_WIDTH {
        if !is_zero_pixel(&image[pixel_index*3], &image[pixel_index*3 + 1], &image[pixel_index*3 + 2]) {
            image[pixel_index*3] = 255;
            image[pixel_index*3 + 1] = 0;
            image[pixel_index*3 + 2] = 0;
        }
    }
}

#[inline]
fn is_zero_pixel(red_value: &u8, green_value: &u8, blue_value: &u8) -> bool {
    *red_value == 0 && *green_value == 0 && *blue_value == 0
}

#[inline]
fn output_image(image: &mut [u8]) {
    let quality = 100;
    let encoder = Encoder::new_file("wallpaper.jpeg", quality).unwrap();

    encoder.encode(&image, IMAGE_WIDTH as u16, IMAGE_HEIGHT as u16, ColorType::Rgb).unwrap();
}