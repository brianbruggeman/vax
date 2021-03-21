//! Browser integration

// /////////////////////////////////////////////////////////////////////
use std::process::{Command, Stdio};
use std::{env, thread, time};

use fantoccini::{Client, ClientBuilder};
use serde_json::json;
use serde_json::map::Map;
use sysinfo::{ProcessExt, Signal, System, SystemExt};
use webdriver::capabilities::Capabilities;

pub(crate) async fn manage_geckodriver() -> Result<(), Box<dyn std::error::Error>> {
    terminate_gecko().await?;
    start_gecko().await?;
    Ok(())
}

async fn terminate_gecko() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Some((pid, process)) = find_gecko().await {
        warn!("Terminating geckodriver is running on pid: {}.", pid);
        process.kill(Signal::Kill);
        warn!("geckodriver({}) is terminated.", pid);
    }
    Ok(())
}

async fn find_gecko() -> Option<(i32, sysinfo::Process)> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.get_processes() {
        if process.name().contains("geckodriver") {
            debug!("Found geckodriver running on pid: {}", pid);
            let process = process.clone();
            let pid = *pid;
            return Some((pid, process));
        }
    }
    None
}

async fn start_gecko() -> Result<(), Box<dyn std::error::Error>> {
    debug!("Starting geckodriver...");
    let host = env::var("WEBDRIVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("WEBDRIVER_PORT").unwrap_or_else(|_| String::from("4444"));
    Command::new("geckodriver")
        .arg("--host")
        .arg(host)
        .arg("--port")
        .arg(port)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("geckodriver command failed to start");
    thread::sleep(time::Duration::from_millis(300));
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Some((_pid, _process)) = find_gecko().await {
    } else {
        error!("Failed to start geckodriver.  Terminating");
        std::process::exit(1);
    }
    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn create_capabilities(_headless: bool, _profile: &str) -> Result<Capabilities, Box<dyn std::error::Error>> {
    // See: https://docs.rs/fantoccini/0.17.3/fantoccini/struct.ClientBuilder.html#method.capabilities
    let always_match_data = [("browserName", json!("firefox")), ("setWindowRect", json!(true)), ("unhandledPromptBehavior", json!("ignore"))];
    // TODO: Figure out how to send in firefox options to fantoccini
    // let first_match_data = [
    //     ("browserVersion", json!("86.0.1")),
    //     ("browserVersion", json!("86.0")),
    //     ("browserVersion", json!("86")),
    // ];
    // let mut firefox_args: Vec<Value> = Vec::new();
    // if headless {
    //     firefox_args.push(json!("-headless"));
    // }
    // if profile != "" {
    //     let profile_path =  {
    //         let p = Path::new(profile);
    //         if p.exists() {
    //             Path::new(&p).to_path_buf()
    //         } else {
    //             warn!("Profile did not exist: {}", &p.display());
    //             let home_path = dirs::home_dir().unwrap();
    //             let profile_path = home_path.join(".vax").join("vax.profile");
    //             std::fs::create_dir_all(profile_path.parent().unwrap())?;
    //             warn!("Using profile: {}", &profile_path.display());
    //             profile_path
    //         }
    //     };
    //     let path = format!("{}", profile_path.display());
    //     firefox_args.push(json!("-profile"));
    //     firefox_args.push(json!(path));
    // }
    // let firefox_options = [
    //     ("args", json!(firefox_args)),
    // ];
    let mut caps = Map::new();
    // TODO:  Replicate fantoccini Client setup so that we can use
    //        both AlwaysMatch and FirstMatch
    // let mut always_match = Map::new();
    for (key, value) in always_match_data.iter() {
        caps.insert(key.to_string(), value.clone());
    }
    // let mut first_match: Vec<Value> = Vec::new();
    // for (key, value) in first_match_data.iter() {
    //     let mut new_data = Map::new();
    //     new_data.insert(key.to_string(), value.clone());
    //     first_match.push(json!(new_data));
    // }
    // caps.insert("alwaysMatch".to_string(), json!(always_match));
    // caps.insert("firstMatch".to_string(), json!(first_match));
    // caps.insert("moz:firefoxOptions".to_string(), json!(firefox_options));
    Ok(caps)
}

pub async fn launch_browser(headless: bool, profile: &str) -> Result<Client, Box<dyn std::error::Error>> {
    manage_geckodriver().await?;
    let start = std::time::Instant::now();
    let host = env::var("WEBDRIVER_HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
    let port = env::var("WEBDRIVER_PORT").unwrap_or_else(|_| String::from("4444"));
    let netloc = format!("http://{}:{}", &host, &port);
    debug!("Connecting to geckodriver at: {}", netloc);
    let c = ClientBuilder::native().capabilities(create_capabilities(headless, profile)?).connect(&netloc).await?;
    let browser_launch = start.elapsed().as_millis();
    info!("Time to launch brower: {} msec", &browser_launch);
    Ok(c)
}

pub async fn goto(url: &str, browser: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    browser.goto(url).await.unwrap_or_else(|why| {
        error!("{}", why);
        error!("Goto failed.");
        std::process::exit(1);
    });
    Ok(())
}
