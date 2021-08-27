use super::IMAGE_HEIGHT;
use super::IMAGE_WIDTH;

struct Pixel {
  row: usize,
  column: usize,
}

impl Pixel {
  #[inline]
  pub fn rotate_left(&mut self, angle: f64) {
    let sin = angle.sin();
    let cos = angle.cos();

    self.row = ((self.row as f64) * cos + (self.row as f64) * sin) as usize;
    self.column = (-(self.column as f64) * sin + (self.column as f64) * cos) as usize;
  }

  #[inline]
  pub fn find_mean(self, other: Pixel) -> Pixel {
    let mut sum = self.vector_sum(other);

    sum.row /= 2;
    sum.column /= 2;

    sum
  }

  #[inline]
  pub fn vector_sum(self, other: Pixel) -> Pixel {
    let sum_row = self.row + other.row;
    let sum_column = self.column + other.column;

    Pixel::new(sum_row, sum_column)
  }

  #[inline]
  pub fn new(row: usize, column: usize) -> Pixel {
    Pixel { row, column }
  }
}

#[inline]
pub fn draw_mask(image: &mut [u8]) {
  for pixel_index in 0..IMAGE_HEIGHT {
    mark_pixel(image, pixel_index, pixel_index);
  }
}

#[inline]
fn mark_pixel(image: &mut [u8], row: usize, column: usize) {
  image[column * 3 + IMAGE_WIDTH * 3 * row] = 255;
  image[column * 3 + IMAGE_WIDTH * 3 * row + 1] = 255;
  image[column * 3 + IMAGE_WIDTH * 3 * row + 2] = 255;
}
