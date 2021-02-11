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
}

fn main() {
    dotenv::dotenv().ok();
    let options = CliOptions::from_args();

    let carrier = options.carrier.to_lowercase();
    let carrier: Carrier = match carrier.as_str() {
        "att" => Carrier::Att(options.phone.clone()),
        "sprint" => Carrier::Sprint(options.phone.clone()),
        "tmobile" => Carrier::Tmobile(options.phone.clone()),
        "verizon" => Carrier::Verizon(options.phone.clone()),
        "" => Carrier::None,
        _ => panic!(
            "Only the following carriers are supported: `att`, `sprint`, `tmobile`, `verizon`"
        ),
    };

    let mut coordinates = Coordinate::new(options.latitude, options.longitude);
    if options.latitude == 0.0 && options.longitude == 0.0 {
        coordinates = find_geo(options.address.clone());
    }

    let available = find_vaccination_locations(coordinates, options.threshold);
    println!(
        "Found `{}` open time slots within `{}` miles of `{}`",
        &available.len(),
        options.threshold,
        &options.address
    );
    let directions_template = format!(
        "https://www.google.com/maps/dir/?api=1&origin={},{}&destination=",
        &coordinates.latitude, &coordinates.longitude
    );
    let messages: Vec<String> = available.iter().map(|(d, l)| {
        let directions_url = format!("{}{},{}", directions_template, l.latitude, l.longitude);
        println!("[miles: {:.0} `{}`] [signup = {}] [directions = {}]", d, l.city, l.url, &directions_url);
        directions_url
    }).collect();

    // if !options.email.is_empty() && !options.phone.is_empty() && carrier != Carrier::None {
    //     messages.iter().for_each(|msg| {
    //         send_text(&msg, &options.email, carrier.clone())
    //     })
    // }
}
