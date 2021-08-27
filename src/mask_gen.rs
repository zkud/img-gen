use super::IMAGE_HEIGHT;
use super::IMAGE_WIDTH;

use super::FIGURE_MAX_HEIGHT;
use super::FIGURE_MAX_WIDTH;
use super::FIGURE_MIN_HEIGHT;
use super::FIGURE_MIN_WIDTH;
use super::FIGURE_ROTATION_COUNT;

#[derive(Clone)]
struct Pixel {
  row: usize,
  column: usize,
}

impl Pixel {
  #[inline]
  pub fn rotate_right(&mut self, angle: f64) {
    let mut centered_row: f64 = self.row as f64 - IMAGE_HEIGHT as f64 / 2.0;
    let mut centered_column: f64 = self.column as f64 - IMAGE_WIDTH as f64 / 2.0;

    let sin = (angle * 3.14 / 180.0).sin();
    let cos = (angle * 3.14 / 180.0).cos();

    centered_row = (centered_row as f64) * cos + (centered_column as f64) * sin;
    centered_column = -(centered_row as f64) * sin + (centered_column as f64) * cos;

    self.row = (centered_row + IMAGE_HEIGHT as f64 / 2.0) as usize;
    self.column = (centered_column + IMAGE_WIDTH as f64 / 2.0) as usize;
  }

  #[inline]
  pub fn find_mean(&self, other: &Pixel) -> Pixel {
    let mut sum = self.vector_sum(other);

    sum.row /= 2;
    sum.column /= 2;

    sum
  }

  #[inline]
  pub fn vector_sum(&self, other: &Pixel) -> Pixel {
    let sum_row = self.row + other.row;
    let sum_column = self.column + other.column;

    Pixel::new(sum_row, sum_column)
  }

  #[inline]
  pub fn find_distance(&self, other: &Pixel) -> f64 {
    let row_diff: f64 = self.row as f64 - other.row as f64;
    let column_diff: f64 = self.column as f64 - other.column as f64;

    (row_diff * row_diff + column_diff * column_diff).sqrt()
  }

  #[inline]
  pub fn row(&self) -> usize {
    self.row
  }

  #[inline]
  pub fn column(&self) -> usize {
    self.column
  }

  #[inline]
  pub fn new(row: usize, column: usize) -> Pixel {
    Pixel { row, column }
  }
}

#[inline]
pub fn draw_mask(image: &mut [u8]) {
  let rotation_angle = 30.0 / (FIGURE_ROTATION_COUNT as f64);

  let mut left_up_base_point = Pixel::new(FIGURE_MIN_HEIGHT, FIGURE_MIN_WIDTH);
  let mut right_up_base_point = Pixel::new(FIGURE_MIN_HEIGHT, FIGURE_MAX_WIDTH);
  let mut left_down_base_point = Pixel::new(FIGURE_MAX_HEIGHT, FIGURE_MIN_WIDTH);
  let mut right_down_base_point = Pixel::new(FIGURE_MAX_HEIGHT, FIGURE_MAX_WIDTH);

  for _ in 0..FIGURE_ROTATION_COUNT/2 {
    left_up_base_point.rotate_right(-rotation_angle);
    right_up_base_point.rotate_right(-rotation_angle);
    left_down_base_point.rotate_right(-rotation_angle);
    right_down_base_point.rotate_right(-rotation_angle);
  }

  draw_figure(
    image,
    left_up_base_point.clone(),
    right_up_base_point.clone(),
    left_down_base_point.clone(),
    right_down_base_point.clone(),
  );

  for _ in 0..FIGURE_ROTATION_COUNT {
    left_up_base_point.rotate_right(rotation_angle);
    right_up_base_point.rotate_right(rotation_angle);
    left_down_base_point.rotate_right(rotation_angle);
    right_down_base_point.rotate_right(rotation_angle);

    draw_figure(
      image,
      left_up_base_point.clone(),
      right_up_base_point.clone(),
      left_down_base_point.clone(),
      right_down_base_point.clone(),
    );
  }
}

fn draw_figure(image: &mut [u8], mut lu_bp: Pixel, mut ru_bp: Pixel, mut ld_bp: Pixel, mut rd_bp: Pixel) {
  while lu_bp.find_distance(&ru_bp) > 10.0 {
    draw_box(image, &lu_bp, &ru_bp, &ld_bp, &rd_bp);

    let mean_lu_ld = lu_bp.find_mean(&ld_bp);
    let mean_lu_ru = lu_bp.find_mean(&ru_bp);
    let mean_ru_rd = ru_bp.find_mean(&rd_bp);
    let mean_ld_rd = ld_bp.find_mean(&rd_bp);
    draw_box(image, &mean_lu_ld, &mean_lu_ru, &mean_ld_rd, &mean_ru_rd);

    lu_bp = mean_lu_ld.find_mean(&mean_lu_ru);
    ru_bp = mean_lu_ru.find_mean(&mean_ru_rd);
    ld_bp = mean_lu_ld.find_mean(&mean_ld_rd);
    rd_bp = mean_ld_rd.find_mean(&mean_ru_rd);
  }
}

#[inline]
fn draw_box(image: &mut [u8], lu_bp: &Pixel, ru_bp: &Pixel, ld_bp: &Pixel, rd_bp: &Pixel) {
  draw_line(image, lu_bp, ru_bp);
  draw_line(image, ru_bp, rd_bp);
  draw_line(image, rd_bp, ld_bp);
  draw_line(image, ld_bp, lu_bp);
}

// Uses Bresenham's line algorithm
#[inline]
fn draw_line(image: &mut [u8], pixel_a: &Pixel, pixel_b: &Pixel) {
  if pixel_a.column() == pixel_b.column() {
    return draw_vertical_line(image, pixel_a, pixel_b);
  }

  if pixel_a.row() == pixel_b.row() {
    return draw_horizontal_line(image, pixel_a, pixel_b);
  }

  draw_oblique_line(image, pixel_a, pixel_b)
}

#[inline]
fn draw_vertical_line(image: &mut [u8], pixel_a: &Pixel, pixel_b: &Pixel) {
  let max_row = if pixel_a.row() >= pixel_b.row() {
    pixel_a.row()
  } else {
    pixel_b.row()
  };
  let min_row = if pixel_a.row() < pixel_b.row() {
    pixel_a.row()
  } else {
    pixel_b.row()
  };

  for row in min_row..max_row + 1 {
    mark_pixel(image, row, pixel_a.column());
  }
}

#[inline]
fn draw_horizontal_line(image: &mut [u8], pixel_a: &Pixel, pixel_b: &Pixel) {
  let max_column = if pixel_a.column() >= pixel_b.column() {
    pixel_a.column()
  } else {
    pixel_b.column()
  };
  let min_column = if pixel_a.column() < pixel_b.column() {
    pixel_a.column()
  } else {
    pixel_b.column()
  };

  for column in min_column..max_column + 1 {
    mark_pixel(image, pixel_a.row(), column);
  }
}

#[inline]
fn draw_oblique_line(image: &mut [u8], pixel_a: &Pixel, pixel_b: &Pixel) {
  mark_pixel(image, pixel_a.row(), pixel_a.column());
  mark_pixel(image, pixel_b.row(), pixel_b.column());

  let mut a_row: f64 = pixel_a.row() as f64;
  let mut a_column: f64 = pixel_a.column() as f64;

  let b_row: f64 = pixel_b.row() as f64;
  let b_column: f64 = pixel_b.column() as f64;

  let row_delta = b_row - a_row;
  let column_delta = b_column - a_column;

  let distance = pixel_a.find_distance(pixel_b);

  for _ in 0..distance as usize {
    a_row += row_delta / distance;
    a_column += column_delta / distance;

    mark_pixel(image, a_row as usize, a_column as usize);
  }
}

#[inline]
fn mark_pixel(image: &mut [u8], row: usize, column: usize) {
  image[column * 3 + IMAGE_WIDTH * 3 * row] = 255;
  image[column * 3 + IMAGE_WIDTH * 3 * row + 1] = 255;
  image[column * 3 + IMAGE_WIDTH * 3 * row + 2] = 255;
}
