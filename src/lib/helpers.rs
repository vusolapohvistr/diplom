use super::{distance::Distance, sphere_point::SpherePoint};


pub fn calculate_path_length(points: &Vec<SpherePoint>) -> f64 {
  let mut result = 0.0;

  for i in 0..points.len() - 1 {
    result += points[i].distance_to(&points[i + 1]);
  }

  result += points.last().unwrap().distance_to(&points[0]);
  
  result
}
