pub trait Distance {
  fn distance_to(&self, point: &Self) -> f64;
}
