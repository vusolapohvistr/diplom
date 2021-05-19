use std::f64::consts::PI;

use rand::Rng;

use super::{discrete_point::DiscretePoint, sphere_point::SpherePoint};

pub fn random_sphere_point() -> SpherePoint {
  let mut thread_rng = rand::thread_rng();
  SpherePoint::from_radians(
    thread_rng.gen::<f64>() * PI * 2.0 - PI,
    thread_rng.gen::<f64>() * PI - PI / 2.0,
  )
}

pub fn random_sphere_points(n: usize) -> Vec<SpherePoint> {
  (0..n)
    .map(|_| random_sphere_point())
    .collect()
}

pub fn random_discrete_point() -> DiscretePoint {
  let mut thread_rng = rand::thread_rng();
  DiscretePoint::new(
    thread_rng.gen(),
  )
}

pub fn random_discrete_points(n: usize) -> Vec<DiscretePoint> {
  (0..n)
    .map(|_| random_discrete_point())
    .collect()
}
