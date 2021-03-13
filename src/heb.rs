//! Contains logic for accessing H.E.B's Vaccination api
//!

// /////////////////////////////////////////////////////////////////////
use std::{thread, time};
use std::iter::Iterator;

use fantoccini::{elements::Element, Client, Locator};
use geoutils::Location;

use rand::{rngs::ThreadRng, Rng};
use reqwest::get;
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

/// Finds the locations currently open within the specified threshold
///   and orders by distance from the home address.
pub async fn find_vaccination_locations(home_coordinates: &Coordinate, distance_threshold: u16) -> Result<Vec<(f64, HebLocation)>, Box<dyn std::error::Error>> {
    let source = Location::new(home_coordinates.latitude, home_coordinates.longitude);
    let default = Ok(Vec::new());
    let url = std::env::var("URL").unwrap_or_else(|_| URL.to_string());
    match get(url).await {
        Err(_) => default,
        Ok(r) => match r.text().await {
            Err(_) => default,
            Ok(response) => match serde_json::from_str::<HebResponse>(&response) {
                Err(_) => default,
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
                        .collect();
                    locations.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    Ok(locations)
                }
            },
        },
    }
}

/// Automatically selects date and time
pub async fn auto_signup(url: &str, headless: bool, profile: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = goto(url, headless, profile).await?;
    let page_1 = handle_page_1(&mut browser).await?;
    if !page_1 {
        return Ok(())
    }
    wait_for_human_entry(&mut browser).await;
    Ok(())
}

async fn wait_for_human_entry(mut browser: &mut Client) {
    let start = time::Instant::now();
    let end = time::Duration::from_secs(60 * 10);
    let poll_time = time::Duration::from_secs(1);
    loop {
        if start.elapsed().as_secs() >= end.as_secs() {
            break;
        }
        let gone = detect_empty(&mut browser).await;
        if gone { break; }
        let error = detect_error(&mut browser).await;
        if error { break; }
        thread::sleep(poll_time);
    }
}

async fn handle_page_1(mut browser: &mut Client) -> Result<bool, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    // Loop until the website is in a good condition for processing; the javascript takes some time to load from the
    // remote service and is not completely available until after some time.
    let mut selected_shot = false;
    let mut selected_time = false;
    let mut selected_date = false;
    loop {
        if detect_error(&mut browser).await {
            return Ok(false);
        }
        let comboboxes = browser.find_all(Locator::Css("lightning-combobox > div > lightning-base-combobox > div")).await?;
        if comboboxes.len() < 3 {
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        for (index, element) in comboboxes.iter().enumerate() {
            let mut element = element.clone();
            click(&mut element, &mut rng).await?;
            let errors_found = detect_error(&mut browser).await;
            if errors_found {
                return Ok(false);
            }
            if index == 0 && !selected_shot {
                // The first element represents the type of shot
                // Any is perfectly acceptible for right now for the type of shot.
                // TODO: allow for selection
                let mut e = browser.find(Locator::Css("lightning-combobox")).await?;
                click(&mut e, &mut rng).await?;
                selected_shot = true;
            } else if index == 1 && !selected_date {
                // The second element represents the available dates for the shot
                let mut date_combobox_items = browser.find_all(Locator::Css("div[id=\"dropdown-element-14\"] > lightning-base-combobox-item")).await?;
                if date_combobox_items.is_empty() {
                    std::thread::sleep(std::time::Duration::from_millis(210));
                    date_combobox_items = browser.find_all(Locator::Css("div[id=\"dropdown-element-14\"] > lightning-base-combobox-item")).await?;
                }
                let mut dates: Vec<String> = Vec::new();
                let mut date_found = false;
                for mut item in date_combobox_items {
                    for mut span in item.find_all(Locator::Css("span[class=\"slds-truncate\"]")).await? {
                        let text = span.text().await?;
                        if !text.is_empty() {
                            date_found = true;
                            dates.push(text);
                        }
                    }
                    if date_found {
                        click(&mut item, &mut rng).await?;
                        selected_date = true;
                        break;
                    }
                }
            } else if index == 2 && !selected_time {
                // The third element represents the available times for the date selected
                let mut date_combobox_items = browser.find_all(Locator::Css("div[id=\"dropdown-element-18\"] > lightning-base-combobox-item")).await?;
                while date_combobox_items.is_empty() {
                    date_combobox_items = browser.find_all(Locator::Css("div[id=\"dropdown-element-14\"] > lightning-base-combobox-item")).await?;
                }
                let mut dates: Vec<String> = Vec::new();
                let mut date_found = false;
                for mut item in date_combobox_items {
                    for mut span in item.find_all(Locator::Css("span[class=\"slds-truncate\"]")).await? {
                        let text = span.text().await?;
                        if !text.is_empty() {
                            date_found = true;
                            dates.push(text);
                        }
                    }
                    if date_found {
                        click(&mut item, &mut rng).await?;
                        selected_time = true;
                        break;
                    }
                }
            }
        }
        let source_contains_error = detect_error(&mut browser).await;
        if source_contains_error {
            return Ok(false)
        }
        let source = &browser.source().await?;
        if source.contains("Continue") && selected_shot && selected_date && selected_time {
            let file_path = std::path::Path::new("last-source.html");
            let source = browser.source().await?;
            let contents = source.as_bytes();
            std::fs::write(&file_path, &contents)?;

            let mut button = browser.find(Locator::Css("[title=\"Continue\"]")).await?;
            click(&mut button, &mut rng).await?;
            let source_contains_error = detect_error(&mut browser).await;
            if source_contains_error {
                return Ok(false)
            } else {
                return Ok(true)
            }
        }
    }
}

pub async fn detect_error(browser: &mut Client) -> bool {
    let errors = [
        "This timeslot is full.",
        "Appointments are no longer available for this location.",
        "We could not verify that you are a human. Please try again from a different browser, computer, internet connection, or at another time.",
    ];
    let source = browser.source().await.unwrap_or_default();
    for (index, error) in errors.iter().enumerate() {
        if source.contains(error) {
            if index == 2 {
                error!("{}", error);
                error!("IP address is flagged.  VPN must be restarted.");
                std::process::exit(1);
            } else {
                warn!("{}", error);
                return true;
            }
        }
    }
    false
}

pub async fn click(element: &mut Element, rng: &mut ThreadRng) -> Result<(), Box<dyn std::error::Error>> {
    let min_click: u64 = 75;
    let max_click: u64 = 300;
    let wait_ms: u64 = rng.gen_range(min_click..max_click);
    let duration = std::time::Duration::from_millis(wait_ms);
    std::thread::sleep(duration);
    element.clone().click().await?;
    Ok(())
}

pub async fn detect_empty(browser: &mut Client) -> bool {
    let source = browser.source().await.unwrap_or_default();
    source == *""
}