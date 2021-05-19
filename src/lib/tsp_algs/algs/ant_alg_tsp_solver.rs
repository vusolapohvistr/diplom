use std::collections::HashSet;

use rand::Rng;

use crate::lib::{distance::Distance, tsp_algs::tsp_alg::TSPSolver};

pub struct AntAlgTSPSolver {
  alfa: f64,
  beta: f64,
  ro: f64,
  iters_count: usize,
  ant_capacity: f64,
  initial_pheromones_val: f64,
}

impl AntAlgTSPSolver {
  pub fn new(
    alfa: f64,
    beta: f64,
    ro: f64,
    iters_count: usize,
    ant_capacity: f64,
    initial_pheromones_val: f64
  ) -> Self {
    AntAlgTSPSolver {
      alfa,
      beta,
      ro,
      iters_count,
      ant_capacity,
      initial_pheromones_val,
    }
  }
}

impl TSPSolver for AntAlgTSPSolver {
  fn solve_tsp<T: Distance + Clone>(&self, points: &Vec<T>, start_point_idx: usize) -> Vec<T> {
    let distances_mat: Vec<Vec<f64>> = (0..points.len())
      .map(|a| 
        (0..points.len())
        .map(move |b| points[a].distance_to(&points[b]))
        .collect()
      )
      .collect();

    let mut pheromones_mat = vec![vec![self.initial_pheromones_val; points.len()]; points.len()];

    let mut shortest_path: Vec<usize> = Vec::new();
    let mut shortest_path_length = f64::MAX;

    let mut rand = rand::thread_rng();
    // each iteration is ant simulation
    for _ in 0..self.iters_count {
      let mut current_point = start_point_idx;
      let mut visited_points: HashSet<usize> = HashSet::new();

      let mut current_path: Vec<usize> = Vec::with_capacity(points.len());
      current_path.push(start_point_idx);

      let mut current_path_length = 0.0;

      while visited_points.len() < points.len() {
        visited_points.insert(current_point);

        let mut balance_sum = 0.0;

        for i in 0..distances_mat.len() {
          if distances_mat[current_point][i].eq(&0.0) || visited_points.contains(&i) {
            continue;
          }

          balance_sum += pheromones_mat[current_point][i].powf(self.alfa)
            * (1.0 / distances_mat[current_point][i]).powf(self.beta);
        }

        for i in 0..points.len() {
          if distances_mat[current_point][i].eq(&0.0) {
            continue;
          }

          let prob_to_move: f64 = 
            if visited_points.contains(&i) {
              0.0
            } else {
              pheromones_mat[current_point][i].powf(self.alfa) *
              (1.0 / distances_mat[current_point][i]).powf(self.beta) / balance_sum 
            };

          let random_number: f64 = rand.gen();

          if random_number < prob_to_move {
            current_path.push(i);

            current_path_length += distances_mat[current_point][i];
            current_point = i;
            break;
          }
        }
      }

      current_path.push(start_point_idx);
      current_path_length += distances_mat[current_point][start_point_idx];

      // leaving pheromones
      for i in 0..current_path.len() - 1 {
        pheromones_mat[current_path[i]][current_path[i + 1]] +=
          self.ant_capacity / current_path_length;
      }

      if current_path_length < shortest_path_length {
        shortest_path = current_path;
        shortest_path_length = current_path_length;
      }

      // vaporizing pheromones
      for i in pheromones_mat.iter() {
        for mut g in i.iter() {
          g = &(g * (1.0 - self.ro));
        }
      }
    }

    shortest_path.into_iter().map(|index| points[index].clone()).collect()
  }
}
