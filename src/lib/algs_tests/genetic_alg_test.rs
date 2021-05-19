use std::io::Write;

use crate::{lib::{data_generators::random_sphere_points, distance::Distance, helpers::calculate_path_length, tsp_algs::{algs::genetic_alg_tsp_solver::GeneticAlgTSPSolver, tsp_alg::TSPSolver}}};

pub fn genetic_alg_test<T: Distance + Clone>(test_data: &Vec<Vec<T>>, file_prefix: &str) {
  let population_size = 100;
  let dying_rate = 0.2;
  let mut_rate = 0.1;
  let crossover_rate = 0.8;

  for points in test_data {
    println!("Start ga {} test. Points length: {}", file_prefix, points.len());

    let write_file = std::fs::File::create(
      format!("{}_ga_test_{}.csv", file_prefix, points.len())
    ).unwrap();
    let mut writer = std::io::BufWriter::new(&write_file);

    writeln!(&mut writer, "gens_count,path_length,time_elapsed");

    for gens_count in (10..=1000).step_by(10) {
      let now = std::time::Instant::now();

      let solver = GeneticAlgTSPSolver::new(
        population_size,
        gens_count,
        crossover_rate,
        mut_rate,
        dying_rate
      );
      let result = solver.solve_tsp(&points, 0);

      let elapsed = now.elapsed();

      let path_length = calculate_path_length(&result);

      writeln!(&mut writer, "{},{},{}", gens_count, path_length, elapsed.as_millis());
    }
  }
}
