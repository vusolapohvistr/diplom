#![allow(dead_code)]

use core::f64;

use lib::{algs_tests::{ant_alg_test::ant_alg_test, genetic_alg_test::{self, genetic_alg_test}}, data_generators::random_sphere_points, distance::Distance, sphere_point::SpherePoint, tsp_algs::{ant_alg::ant_alg_tsp_solver::AntAlgTSPSolver, tsp_alg::TSPSolver}};

use crate::lib::tsp_algs::ant_alg::genetic_alg_tsp_solver::GeneticAlgTSPSolver;


mod lib;

fn main() {
  genetic_alg_test();
  ant_alg_test();
}
