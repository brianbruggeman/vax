//! Command-line options

// /////////////////////////////////////////////////////////////////////
use structopt::StructOpt;

/// Command-line options
#[derive(Debug, StructOpt)]
#[structopt(name = "vax", about = "A Covid-19 Vaccination Signup Tool")]
pub struct CliOptions {
    // The number of occurrences of the `verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,

    /// Physical address of individual needing a vaccine
    #[structopt(short, long, env = "ADDRESS", default_value = "")]
    pub address: String,

    /// Latitude of the physical address for individual needing a vaccine
    #[structopt(long, env = "LATITUDE", default_value = "0.0")]
    pub latitude: f64,

    /// Longitude of the physical address for individual needing a vaccine
    #[structopt(long, env = "LONGITUDE", default_value = "0.0")]
    pub longitude: f64,

    /// Only display sign-up link; do not open browser
    #[structopt(long)]
    pub hide_signup: bool,

    /// Open map
    #[structopt(long)]
    pub map: bool,

    /// Run in headless mode
    #[structopt(long)]
    pub headless: bool,

    /// Disables extra logic to fake out recaptcha    
    #[structopt(long)]
    pub fast: bool,

    /// Auto select time and date
    #[structopt(long)]
    pub auto: bool,

    /// Set the Firefox profile
    #[structopt(long)]
    pub profile: Option<String>,

    /// Threshold in miles for how far to travel to get vaccine
    #[structopt(short = "t", long, env = "THRESHOLD", default_value = "20")]
    pub threshold: u16,

    /// How often to ping H.E.B's API in milliseconds
    #[structopt(short = "T", long, env = "TIMEOUT", default_value = "350")]
    pub timeout: u64,
}

impl CliOptions {
    pub fn coordinates(&self) -> crate::Coordinate {
        crate::Coordinate::new(self.latitude, self.longitude)
    }
}

// clippy doesn't understand the macro expansion from above which requires a result value
#[allow(clippy::unnecessary_wraps)]
pub fn from_string_bool(value: &str, default: bool) -> bool {
    let lc_value: String = value.to_lowercase();
    match lc_value.as_str() {
        "t" | "true" | "1" | "on" | "yes" | "y" => true,
        "f" | "false" | "0" | "off" | "no" | "n" => false,
        _ => {
            warn!("Could not process boolean value: `{}`.  Setting to default: `{}`", value, default);
            default
        }
    }
}
