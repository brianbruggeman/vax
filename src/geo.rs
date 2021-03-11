//! Methods for accessing geo
//!

// /////////////////////////////////////////////////////////////////////
use geocoding::{Forward, Openstreetmap, Point};

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
pub fn find_geo(address: String) -> Coordinate {
    let osm = Openstreetmap::new();
    let resource: Vec<Point<f64>> = osm.forward(&address).unwrap();
    let point = match resource.get(0) {
        Some(p) => p,
        None => {
            eprintln!("Could not match address: '{}'", &address);
            std::process::exit(1);
        }
    };
    Coordinate {
        latitude: point.0.y,
        longitude: point.0.x,
    }
}
