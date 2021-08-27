use super::IMAGE_HEIGHT;
use super::IMAGE_WIDTH;

#[inline]
pub fn fill_with_colors(image: &mut [u8]) {
  for pixel_index in 0..IMAGE_HEIGHT * IMAGE_WIDTH {
    if !is_zero_pixel(
      &image[pixel_index * 3],
      &image[pixel_index * 3 + 1],
      &image[pixel_index * 3 + 2],
    ) {
      image[pixel_index * 3] = 200;
      image[pixel_index * 3 + 1] = 10;
      image[pixel_index * 3 + 2] = 10;
    }
  }
}

#[inline]
fn is_zero_pixel(red_value: &u8, green_value: &u8, blue_value: &u8) -> bool {
  *red_value == 0 && *green_value == 0 && *blue_value == 0
}
