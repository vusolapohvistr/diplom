#![allow(dead_code)]

use core::f64;

use lib::{algs_tests::{ant_alg_test::ant_alg_test, genetic_alg_test::{self, genetic_alg_test}}, data_generators::{random_discrete_points, random_sphere_points}, distance::Distance, sphere_point::SpherePoint, tsp_algs::{algs::ant_alg_tsp_solver::AntAlgTSPSolver, tsp_alg::TSPSolver}};

use crate::lib::tsp_algs::algs::genetic_alg_tsp_solver::GeneticAlgTSPSolver;


mod lib;

fn main() {
  let N: [usize; 3] = [10, 100, 500];

  // let test_data_discrete = N
  //   .iter()
  //   .map(|size| random_discrete_points(*size))
  //   .collect();

  // println!("Start discrete test");
  // genetic_alg_test(&test_data_discrete, "discrete");
  // ant_alg_test(&test_data_discrete, "discrete");

  let test_data_sphere = N
    .iter()
    .map(|size| random_sphere_points(*size))
    .collect();

  println!("Start sphere test");
  genetic_alg_test(&test_data_sphere, "sphere");
  ant_alg_test(&test_data_sphere, "sphere");
}
