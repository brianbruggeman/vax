use std::thread::sleep;
use std::time::Duration;
use std::env::var;

use chrono::Local;
use structopt::StructOpt;

use vax::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "vax", about = "A Covid-19 Vaccination Signup Tool")]
struct CliOptions {
    /// Physical address of individual needing a vaccine
    #[structopt(short, long, env = "ADDRESS", default_value = "")]
    address: String,

    /// Latitude of the physical address for individual needing a vaccine
    #[structopt(long, env = "LATITUDE", default_value = "0.0")]
    latitude: f64,

    /// Longitude of the physical address for individual needing a vaccine
    #[structopt(long, env = "LONGITUDE", default_value = "0.0")]
    longitude: f64,

    /// Phone carrier
    #[structopt(short, long, env = "CARRIER", default_value = "")]
    carrier: String,

    /// Email address of program user
    #[structopt(short, long, env = "EMAIL", default_value = "")]
    email: String,

    /// Phone number of individual needing a vaccine
    #[structopt(short, long, env = "PHONE", default_value = "")]
    phone: String,

    /// Threshold in miles for how far to travel to get vaccine
    #[structopt(short, long, env = "THRESHOLD", default_value = "25")]
    threshold: u16,

    /// How often to ping H.E.B's API in milliseconds
    #[structopt(short, long, env = "TIMEOUT", default_value = "10000")]
    timeout: u64,
}

fn main() {
    let dotenv_filename = var("DOTENV_FILE").unwrap_or_else(|_| ".env".to_string());
    dotenv::from_filename(&dotenv_filename).ok();
    let options = CliOptions::from_args();

    let mut coordinates = Coordinate::new(options.latitude, options.longitude);
    if options.latitude == 0.0 && options.longitude == 0.0 {
        coordinates = find_geo(options.address.clone());
    }

    let mut already_found: Vec<HebLocation> = Vec::new();

    loop {
        let available = find_vaccination_locations(coordinates, options.threshold);
        println!(
            "{}: Found `{}` open time slots within `{}` miles of `{}`",
            Local::now(),
            &available.len(),
            options.threshold,
            &options.address
        );
        let directions_template = format!(
            "https://www.google.com/maps/dir/?api=1&origin={},{}&destination=",
            &coordinates.latitude, &coordinates.longitude
        );
        available
            .iter()
            .map(|(d, l)| {
                let directions_url = format!("{}{},{}", directions_template, l.latitude, l.longitude);
                println!(
                    "[miles: {:.0} `{}`] [signup = {}] [directions = {}]",
                    d, l.city, l.url, &directions_url
                );
                if !already_found.contains(&l) {
                    webbrowser::open(&directions_url).unwrap();
                    webbrowser::open(&l.url).unwrap();
                    already_found.push(l.clone());
                }
                directions_url
            })
            .filter(|_| {
                !options.email.is_empty()
                    && !options.phone.is_empty()
                    && options.carrier != "".to_string()
            })
            .for_each(|msg| send_text(&msg, &options.email, &options.phone, &options.carrier));
        sleep(Duration::from_millis(options.timeout));
    }
}
