use std::f64::consts::PI;

use super::distance::Distance;

#[derive(Debug, Clone)]
pub struct SpherePoint {
  // in radians
  lon: f64,
  // in radians
  lat: f64,
}

impl SpherePoint {
  pub fn from_degrees(lon: f64, lat: f64) -> Self {
    SpherePoint {
      lon: degrees_to_radians(lon),
      lat: degrees_to_radians(lat),
    }
  }

  pub fn from_radians(lon: f64, lat: f64) -> Self {
    SpherePoint {
      lon,
      lat,
    }
  }
}

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
  (degrees * PI) / 180.0
}

impl Distance for SpherePoint {
  fn distance_to(&self, point: &Self) -> f64 {
    let d_lat = point.lat - self.lat;
    let d_lon = point.lon - self.lon;

    let a = 
      (d_lat / 2.0).sin().powi(2)
      +
      (d_lon / 2.0).sin().powi(2) * self.lat.cos() * point.lat.cos();


    a.sqrt().asin()
  }
}
