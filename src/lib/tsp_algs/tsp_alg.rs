use crate::lib::distance::Distance;

pub trait TSPSolver {
  fn solve_tsp<T: Distance + Clone>(&self, points: &Vec<T>, start_point_idx: usize) -> Vec<T>;
}
