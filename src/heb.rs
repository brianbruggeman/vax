//! Contains logic for accessing H.E.B's Vaccination api
//!

// /////////////////////////////////////////////////////////////////////
use geoutils::Location;
use reqwest::blocking::get;
use serde::{Deserialize, Deserializer, Serialize};

const METERS_TO_MILES: f64 = 0.000621371;

use crate::*;

// This contains the json data of the vaccination information
const URL: &str = "https://heb-ecom-covid-vaccine.hebdigital-prd.com/vaccine_locations.json";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HebLocation {
    #[serde(deserialize_with = "deserialize_null_default")]
    pub url: String,
    #[serde(rename = "street", deserialize_with = "deserialize_null_default")]
    pub address: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub city: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub state: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub zip: String,
    #[serde(rename = "openTimeslots")]
    open_time_slots: u32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub latitude: f64,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub longitude: f64,
}

impl std::fmt::Display for HebLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("[{}] {}, {}, {} {} [{}]", self.open_time_slots, self.address, self.city, self.state, self.zip, self.url);
        write!(f, "{}", output)
    }
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HebResponse {
    locations: Vec<HebLocation>,
}

// Sometimes the system sends us to a page that says
// "Appointments are no longer available."  This is an effort to remove
//   False positives.  This won't catch everything, but it does minimize
//   However, it also may create latency for the end user and that will
//   maybe cause the user to fail when they otherwise may have succeeded
fn check_availability(url: &str) -> bool {
    match get(url) {
        Err(_) => false,
        Ok(r) => match r.text() {
            Err(_) => false,
            Ok(r) => !r.contains("Appointments are no longer available for this location."),
        },
    }
}

/// Finds the locations currently open within the specified threshold
///   and orders by distance from the home address.
pub fn find_vaccination_locations(home_coordinates: Coordinate, distance_threshold: u16) -> Vec<(f64, HebLocation)> {
    let source = Location::new(home_coordinates.latitude, home_coordinates.longitude);
    match get(URL) {
        Err(_) => Vec::new(),
        Ok(r) => match r.text() {
            Err(_) => Vec::new(),
            Ok(response) => match serde_json::from_str::<HebResponse>(&response) {
                Err(_) => Vec::new(),
                Ok(heb_response) => {
                    let mut locations: Vec<(f64, HebLocation)> = heb_response
                        .locations
                        .iter()
                        .filter(|loc| loc.open_time_slots > 0 && loc.latitude != 0.0 && loc.longitude != 0.0)
                        .map(|loc| {
                            let destination = Location::new(loc.latitude, loc.longitude);
                            let distance = source.distance_to(&destination).unwrap();
                            let meters = distance.meters();
                            let miles = meters * METERS_TO_MILES;
                            (miles, loc.clone())
                        })
                        .filter(|(d, _)| (distance_threshold > 0 && *d <= distance_threshold as f64) || distance_threshold == 0)
                        .filter(|(_, h)| check_availability(&h.url))
                        .collect();
                    locations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    locations
                }
            },
        },
    }
}