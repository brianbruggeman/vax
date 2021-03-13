//! Command-line utilities
use std::path::Path;
use std::{env, fs};

use dialoguer::{theme::ColorfulTheme, Input};
use structopt::StructOpt;

use crate::{find_geo, Coordinate};

use super::*;

/// Initializes command-line interface
pub async fn init() -> Result<CliOptions, Box<dyn std::error::Error>> {
    let dotenv_filename = env::var("DOTENV_FILE").unwrap_or_else(|_| ".env".to_string());
    let dotenv_path = Path::new(&dotenv_filename);
    dotenv::from_path(&dotenv_path).ok();
    let mut options = CliOptions::from_args();

    let level = match options.verbose {
        x if x > 1 => "vax=debug",
        x if x > 0 => "vax=info",
        _ => "vax=warn",
    };
    match env::var("RUST_LOG") {
        Ok(v) => {
            let new_value = format!("{},{}", &v, &level);
            env::set_var("RUST_LOG", &new_value);
        }
        Err(_) => {
            let new_value = level.to_string();
            env::set_var("RUST_LOG", &new_value);
        }
    }

    log::debug!("RUST_LOG: {}", env::var("RUST_LOG").unwrap());
    env_logger::init();

    check_address(&mut options, &dotenv_path).await?;
    check_map(&mut options).await?;
    check_hide_signup(&mut options).await?;

    check_coordinates(&mut options).await?;

    check_headless(&mut options).await?;

    check_auto(&mut options).await?;
    Ok(options)
}

/// Checks for address in command-line options and prompts if address
///  isn't found
pub async fn check_address(options: &mut CliOptions, dotenv_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dotenv_filename = dotenv_path.file_name().unwrap();
    if options.address == *"" {
        options.address = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Address needed: (e.g. 1234 Street, City, State, Zip)")
            .interact_text()
            .unwrap();
        let data = match &dotenv_path.exists() {
            true => fs::read_to_string(&dotenv_filename).expect("Unable to read file"),
            false => String::new(),
        };
        let mut data = data.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
        data.push(format!("ADDRESS=\"{}\"", &options.address));
        let data: String = data.join("\n");
        fs::write(&dotenv_filename, &data.as_bytes()).expect("Unable to write file");
    }
    debug!("ADDRESS={}", &options.address);
    Ok(())
}

pub async fn check_map(options: &mut CliOptions) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(v) = env::var("MAP") {
        let v = from_string_bool(&v, options.map);
        options.map = v;
    }
    debug!("MAP={:?}", &options.map);
    Ok(())
}

pub async fn check_headless(options: &mut CliOptions) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(v) = env::var("HEADLESS") {
        if !options.headless {
            let v = from_string_bool(&v, options.headless);
            options.headless = v;
        }
    }
    debug!("HEADLESS={:?}", &options.headless);
    Ok(())
}

pub async fn check_auto(options: &mut CliOptions) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(v) = env::var("AUTO") {
        if !options.auto {
            let v = from_string_bool(&v, options.auto);
            options.auto = v;
        }
    }
    debug!("AUTO={:?}", &options.auto);
    Ok(())
}

pub async fn check_hide_signup(options: &mut CliOptions) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(v) = env::var("HIDE_SIGNUP") {
        if !options.hide_signup {
            let v = from_string_bool(&v, options.hide_signup);
            options.hide_signup = v;
        }
    }
    debug!("HIDE_SIGNUP={:?}", &options.hide_signup);
    Ok(())
}

pub async fn check_coordinates(options: &mut CliOptions) -> Result<(), Box<dyn std::error::Error>> {
    let mut coordinates = Coordinate::new(options.latitude, options.longitude);
    if options.latitude == 0.0 && options.longitude == 0.0 {
        coordinates = find_geo(options.address.clone()).await?;
    }
    options.latitude = coordinates.latitude;
    options.longitude = coordinates.longitude;
    Ok(())
}
