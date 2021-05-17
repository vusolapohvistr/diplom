use std::io::Write;

use crate::{lib::{data_generators::random_sphere_points, helpers::calculate_path_length, tsp_algs::{ant_alg::ant_alg_tsp_solver::AntAlgTSPSolver, tsp_alg::TSPSolver}}};

pub fn ant_alg_test() {
  let N: [usize; 3] = [10, 100, 500];
  let alfa = 0.5;
  let beta = 0.5;
  let ro = 0.1;
  let ant_capacity = 10.0;
  let initial_pheromones_val = 1.0;

  for points_count in N.iter() {
    let points = random_sphere_points(*points_count);

    let write_file = std::fs::File::create(
      format!("ma_test_{}.csv", points_count)
    ).unwrap();
    let mut writer = std::io::BufWriter::new(&write_file);

    writeln!(&mut writer, "iters_count,path_length,time_elapsed");

    for iters_count in (10..1000).step_by(10) {
      let now = std::time::Instant::now();

      let solver = AntAlgTSPSolver::new(
        alfa,
        beta,
        ro,
        iters_count,
        ant_capacity,
        initial_pheromones_val,
      );
      let result = solver.solve_tsp(&points, 0);

      let elapsed = now.elapsed();

      let path_length = calculate_path_length(&result);

      writeln!(&mut writer, "{},{},{}", iters_count, path_length, elapsed.as_millis());
    }
  }
}
