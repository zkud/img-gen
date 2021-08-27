use super::IMAGE_HEIGHT;
use super::IMAGE_WIDTH;
use super::OUTPUT_FILE_NAME;
use jpeg_encoder::{ColorType, Encoder};

#[inline]
pub fn output_image(image: &mut [u8]) {
  let quality = 100;
  let encoder = Encoder::new_file(OUTPUT_FILE_NAME, quality).unwrap();

  encoder
    .encode(
      &image,
      IMAGE_WIDTH as u16,
      IMAGE_HEIGHT as u16,
      ColorType::Rgb,
    )
    .unwrap();
}
