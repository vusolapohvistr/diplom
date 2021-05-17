use std::f64::consts::PI;

use rand::Rng;

use super::sphere_point::SpherePoint;

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
