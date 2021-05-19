use super::distance::Distance;

#[derive(Debug, Clone)]
pub struct DiscretePoint {
  identifier: usize,
}

impl DiscretePoint {
  pub fn new(identifier: usize) -> Self {
    DiscretePoint {
      identifier,
    }
  }
}

impl Distance for DiscretePoint {
  fn distance_to(&self, point: &Self) -> f64 {
    if self.identifier == point.identifier {
      0.0
    } else {
      ((self.identifier - point.identifier) % 2 + 1) as f64
    }
  }
}
