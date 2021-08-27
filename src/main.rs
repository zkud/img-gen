mod img_filling;
mod img_output;
mod mask_gen;

use fallible_alloc::vec::alloc_with_size;

pub const IMAGE_WIDTH: usize = 8192;
pub const IMAGE_HEIGHT: usize = 4096;
pub const IMAGE_SIZE: usize = 100663296;
pub const OUTPUT_FILE_NAME: &str = "wallpaper.jpeg";
pub const FIGURE_MAX_HEIGHT: usize = 3072;
pub const FIGURE_MIN_HEIGHT: usize = 1024;
pub const FIGURE_MAX_WIDTH: usize = 5120;
pub const FIGURE_MIN_WIDTH: usize = 3072;
pub const FIGURE_ROTATION_COUNT: usize = 30;


fn main() {
  let mut image: Vec<u8> = alloc_with_size(IMAGE_SIZE).unwrap();

  mask_gen::draw_mask(image.as_mut());

  img_filling::fill_with_colors(image.as_mut());

  img_output::output_image(image.as_mut());
}
