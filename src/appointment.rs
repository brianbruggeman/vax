//! A standardized appointment structure
//!

// /////////////////////////////////////////////////////////////////////
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::Shot;

/// Appointment
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Appointment {
    street: String,
    city: String,
    state: String,
    date: String,
    time: String,
    shot: Shot,
}

impl Appointment {
    pub fn new(street: &str, city: &str, state: &str, date: &str, time: &str, shot: &str) -> Self {
        Self {
            street: street.to_string(),
            city: city.to_string(),
            state: state.to_string(),
            date: date.to_string(),
            time: time.to_string(),
            shot: Shot::from_str(shot).unwrap(),
        }
    }
}
