//! Methods for accessing geo
//!

// /////////////////////////////////////////////////////////////////////

use geocoding::{Forward, Openstreetmap, Point};

/// Represents a latitude and longitude coordinate on Earth
#[derive(Copy, Clone, Debug, Default)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

/// Finds latitude and longitude given a physical address
pub async fn find_geo(address: String) -> Result<Coordinate, Box<dyn std::error::Error>> {
    let osm = Openstreetmap::new();
    let resource: Vec<Point<f64>> = osm.forward(&address)?;
    let point = match resource.get(0) {
        Some(p) => p,
        None => {
            error!("Could not match address: '{}'", &address);
            std::process::exit(1);
        }
    };
    let coord = Coordinate {
        latitude: point.0.y,
        longitude: point.0.x,
    };
    debug!("Using coordinates: lat={}, lon={}", coord.latitude, coord.longitude);
    Ok(coord)
}
