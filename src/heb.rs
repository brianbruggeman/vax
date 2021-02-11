//! Contains logic for accessing H.E.B's Vaccination api
//!

// /////////////////////////////////////////////////////////////////////
use geoutils::Location;
use reqwest::blocking::get;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

const METERS_TO_MILES: f64 = 0.000621371;

use crate::*;

// This contains the json data of the vaccination information
const URL: &str = "https://heb-ecom-covid-vaccine.hebdigital-prd.com/vaccine_locations.json";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HebLocation {
    #[serde(deserialize_with = "deserialize_null_default")]
    pub url: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub city: String,
    #[serde(rename = "openTimeslots")]
    open_time_slots: u32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub latitude: f64,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub longitude: f64,
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

/// Finds the locations currently open within the specified threshold
///   and orders by distance from the home address.
pub fn find_vaccination_locations<'a>(
    home_coordinates: Coordinate,
    distance_threshold: u16,
) -> Vec<(f64, HebLocation)> {
    let source = Location::new(home_coordinates.latitude, home_coordinates.longitude);
    let response = get(URL).unwrap().text().unwrap();
    let heb_response: HebResponse = serde_json::from_str(&response).unwrap();
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
        .filter(|(d, _)| {
            (distance_threshold > 0 && *d <= distance_threshold as f64) || distance_threshold == 0
        })
        .collect();
    locations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    locations
}

// def generate_vaccination_messages(home_coordinates, locations):
//     lat, lon = home_coordinates
//     directions_template = f'https://www.google.com/maps/dir/?api=1&origin={lat},{lon}&destination={{}},{{}}'

//     messages = []
//     for distance in sorted(locations):
//         location = locations[distance]
//         msg = f"[miles: {distance:3.0f} `{location['city'].capitalize()}`] {location['name']}: [signup = {location['url']}] [directions = {directions_template.format(location['latitude'], location['longitude'])}]"
//         messages.append(msg)
//         print(f"{msg}")
//     return messages
