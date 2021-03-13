//! The library supporting vaccination signups
//!

// /////////////////////////////////////////////////////////////////////
#[macro_use]
extern crate log;

mod appointment;

mod browser;
mod cli;
mod geo;
mod heb;
mod person;
mod shot;

pub use crate::appointment::*;

pub use crate::browser::*;
pub use crate::cli::*;
pub use crate::geo::*;
pub use crate::heb::*;
pub use crate::person::*;
pub use crate::shot::*;
