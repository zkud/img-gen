mod img_filling;
mod img_output;
mod mask_gen;

pub const IMAGE_WIDTH: usize = 2048;
pub const IMAGE_HEIGHT: usize = 1024;
pub const IMAGE_SIZE: usize = 6291456;
pub const OUTPUT_FILE_NAME: &str = "wallpaper.jpeg";
pub const FIGURE_MAX_HEIGHT: usize = 768;
pub const FIGURE_MIN_HEIGHT: usize = 256;
pub const FIGURE_MAX_WIDTH: usize = 1280;
pub const FIGURE_MIN_WIDTH: usize = 768;
pub const FIGURE_ROTATION_COUNT: usize = 30;

fn main() {
  let mut image: [u8; IMAGE_SIZE] = [0; IMAGE_SIZE];

  mask_gen::draw_mask(&mut image);

  img_filling::fill_with_colors(&mut image);

  img_output::output_image(&mut image);
}
