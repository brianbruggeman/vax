//! Vaccines available for Covid-19

// /////////////////////////////////////////////////////////////////////
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Identifier for vaccine
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Shot {
    /// Any shot available
    Any,
    /// Pfizer shot:  age 16+; 2-shots
    Pfizer,
    /// Moderna shot:  age 18+; 2 shots
    Moderna,
    /// Jonhson&Johnson shot; 1 shot
    JohnsonAndJohnson,
}

impl FromStr for Shot {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_lowercase();
        match input.as_str() {
            "any" => Ok(Shot::Any),
            "pfizer" => Ok(Shot::Pfizer),
            "moderna" => Ok(Shot::Moderna),
            "johnsonandjohnson" => Ok(Shot::JohnsonAndJohnson),
            "j&j" => Ok(Shot::JohnsonAndJohnson),
            _ => Ok(Shot::Any),
        }
    }
}
