use std::env::var;
use std::thread::sleep;
use std::time::Duration;

use ansi_term::Colour;
use chrono::Local;
use rayon::prelude::*;
#[cfg(spin)]
use spinners::{Spinner, Spinners};
use structopt::StructOpt;

use vax::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "vax", about = "A Covid-19 Vaccination Signup Tool")]
struct CliOptions {
    // The number of occurrences of the `verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Physical address of individual needing a vaccine
    #[structopt(short, long, env = "ADDRESS", default_value = "")]
    address: String,

    /// Latitude of the physical address for individual needing a vaccine
    #[structopt(long, env = "LATITUDE", default_value = "0.0")]
    latitude: f64,

    /// Longitude of the physical address for individual needing a vaccine
    #[structopt(long, env = "LONGITUDE", default_value = "0.0")]
    longitude: f64,

    /// Hide signup in browser
    #[structopt(long, env = "HIDE_SIGNUP")]
    hide_signup: bool,

    /// Open map
    #[structopt(long, env = "SHOW_MAP")]
    map: bool,

    /// Threshold in miles for how far to travel to get vaccine
    #[structopt(short = "t", long, env = "THRESHOLD", default_value = "25")]
    threshold: u16,

    /// How often to ping H.E.B's API in milliseconds
    #[structopt(short = "T", long, env = "TIMEOUT", default_value = "10000")]
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

    println!(
        "{}: Searching for open time slots within '{}' miles of '{}'",
        Colour::Blue.paint(format!("{}", Local::now())),
        Colour::Green.paint(format!("{}", options.threshold)),
        Colour::Cyan.paint(&options.address.to_string()),
    );

    loop {
        #[cfg(spin)]
        let sp = Spinner::new(Spinners::Line, "Waiting for new vaccination spots...".into());
        let available = find_vaccination_locations(coordinates, options.threshold);
        #[cfg(spin)]
        sp.stop();
        let new_found: usize = available.iter().filter(|(_d, h)| !&already_found.contains(&h)).map(|(_d, _h)| 1).sum();
        if new_found > 0 {
            println!("done.");
        }
        let directions_template = format!("https://www.google.com/maps/dir/?api=1&origin={},{}&destination=", &coordinates.latitude, &coordinates.longitude);
        already_found.extend({
            if (!available.is_empty() || options.verbose > 0) && new_found > 0 {
                println!(
                    "{}: Found `{}` open time slots within `{}` miles of `{}`",
                    Colour::Blue.paint(format!("{}", Local::now())),
                    Colour::Yellow.paint(format!("{}", available.len())),
                    Colour::Green.paint(format!("{}", options.threshold)),
                    Colour::Cyan.paint(&options.address.to_string()),
                );
            }
            available
                .par_iter()
                .map(|(d, l)| {
                    let directions_url = format!("{}{},{}", directions_template, l.latitude, l.longitude);
                    if !already_found.contains(&l) {
                        println!(
                            "[{}: {} miles] [signup = {}] [directions = {}]",
                            Colour::Purple.paint(format!("{}, {}", l.city.to_string(), l.state.to_string())),
                            Colour::Yellow.paint(format!("{:.0}", d)),
                            Colour::Cyan.paint(l.url.to_string()),
                            Colour::Blue.paint(&directions_url),
                        );
                        if options.map {
                            webbrowser::open(&directions_url).unwrap();
                        }
                        if !options.hide_signup {
                            webbrowser::open(&l.url).unwrap();
                        }
                    }
                    l.clone()
                })
                .collect::<Vec<HebLocation>>()
        });
        sleep(Duration::from_millis(options.timeout));
    }
}
