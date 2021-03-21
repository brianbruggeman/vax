#[macro_use]
extern crate log;
use std::{thread, time};

use ansi_term::Colour;
#[cfg(spin)]
use spinners::{Spinner, Spinners};

use vax::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = init().await?;

    let mut already_found: Vec<HebLocation> = Vec::new();

    info!(
        "Searching for open time slots within '{}' miles of '{}'",
        Colour::Green.paint(format!("{}", options.threshold)),
        Colour::Cyan.paint(&options.address),
    );

    let coord = Coordinate::new(options.latitude, options.longitude);
    let profile = options.profile.unwrap_or_default();
    let mut browser = launch_browser(options.headless, &profile).await?;

    loop {
        #[cfg(spin)]
        let sp = Spinner::new(Spinners::Line, "Waiting for new vaccination spots...".into());
        let available = find_vaccination_locations(&coord, options.threshold).await?;
        #[cfg(spin)]
        sp.stop();
        let new_found: usize = available.iter().filter(|(_d, h)| !&already_found.contains(&h)).map(|(_d, _h)| 1).sum();
        if new_found > 0 {
            info!("done.");
        }
        let directions_template = format!("https://www.google.com/maps/dir/?api=1&origin={},{}&destination=", &options.latitude, &options.longitude);
        if (!available.is_empty() || options.verbose > 0) && new_found > 0 {
            info!(
                "Found `{}` open time slots within `{}` miles of `{}`",
                Colour::Yellow.paint(format!("{}", available.len())),
                Colour::Green.paint(format!("{}", options.threshold)),
                Colour::Cyan.paint(&options.address.to_string()),
            );
        }
        for (d, l) in available {
            let directions_url = format!("{}{},{}", directions_template, l.latitude, l.longitude);
            if !already_found.contains(&l) {
                let signup_message = match (options.map, options.hide_signup, options.auto) {
                    (false, true, false) => {
                        format!(
                            "[{}: {} miles] [signup = {}] [directions = {}]",
                            Colour::Purple.paint(format!("{}, {}", l.city.to_string(), l.state.to_string())),
                            Colour::Yellow.paint(format!("{:.0}", d)),
                            Colour::Cyan.paint(l.url.to_string()),
                            Colour::Blue.paint(&directions_url),
                        )
                    }
                    (true, true, false) => {
                        format!(
                            "[{}: {} miles] [signup = {}]",
                            Colour::Purple.paint(format!("{}, {}", l.city.to_string(), l.state.to_string())),
                            Colour::Yellow.paint(format!("{:.0}", d)),
                            Colour::Cyan.paint(l.url.to_string()),
                        )
                    }
                    (_, false, false) => {
                        format!(
                            "[{}: {} miles]",
                            Colour::Purple.paint(format!("{}, {}", l.city.to_string(), l.state.to_string())),
                            Colour::Yellow.paint(format!("{:.0}", d)),
                        )
                    }
                    (_, _, true) => {
                        format!(
                            "[{}: {} miles] Signup is automated...",
                            Colour::Purple.paint(format!("{}, {}", l.city.to_string(), l.state.to_string())),
                            Colour::Yellow.paint(format!("{:.0}", d)),
                        )
                    }
                };
                info!("{}", &signup_message);
                if options.map {
                    webbrowser::open(&directions_url).unwrap();
                }

                // Execute the future, blocking the current thread until completion
                if options.auto {
                    let url = l.url.clone();
                    auto_signup(&url, &mut browser).await?;
                    info!("Got one.");
                } else if !options.hide_signup {
                    webbrowser::open(&l.url.clone()).unwrap();
                }
            }
            already_found.push(l.clone());
        }
        thread::sleep(time::Duration::from_millis(options.timeout));
    }
}
