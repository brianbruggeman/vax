# Vax
[![continuous integration](https://github.com/brianbruggeman/vax/actions/workflows/ci.yml/badge.svg)](https://github.com/brianbruggeman/vax/actions)


A COVD-19 Signup tool for use with H.E.B's Backend API.

## Quickstart

To run:

```bash
$ cargo run --release
```

Alternative, you can install to your system with:

```bash
$ cargo install --path .
```

And then run:

```
$ vax
```

## Options

The script has multiple command-line options:

```
vax 0.5.0-dev
A Covid-19 Vaccination Signup Tool

USAGE:
    vax [FLAGS] [OPTIONS]

FLAGS:
        --auto           Auto select time and date
    -h, --help           Prints help information
        --headless       Run in headless mode
        --hide-signup    Only display sign-up link; do not open browser
        --map            Open map
    -V, --version        Prints version information
    -v, --verbose        Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
    -a, --address <address>        Physical address of individual needing a vaccine [env: ADDRESS=1201 Rutherford Dr,
                                   Leander, TX, 78641]  [default: ]
        --latitude <latitude>      Latitude of the physical address for individual needing a vaccine [env:
                                   LATITUDE=30.561073489730106]  [default: 0.0]
        --longitude <longitude>    Longitude of the physical address for individual needing a vaccine [env:
                                   LONGITUDE=-97.82630352841902]  [default: 0.0]
        --profile <profile>        Set the Firefox profile
    -t, --threshold <threshold>    Threshold in miles for how far to travel to get vaccine [env: THRESHOLD=20]
                                   [default: 20]
    -T, --timeout <timeout>        How often to ping H.E.B's API in milliseconds [env: TIMEOUT=3000]  [default: 350]
```

This script can be controlled with environment variables or a dotenv file
specified by `DOTENV_FILE` or defaulting to `.env`:

```
# these are also all command-line options
ADDRESS="your address"
# LATITUDE="your home coordinate latitude if you have it"
# LONGITUDE="your home coordinate longitude if you have it"
THRESHOLD="25"
TIMEOUT="1000"
```

# Automation

This program can automate some of the signup process.  To enable, you
must have both [geckodriver](https://github.com/mozilla/geckodriver/releases) and 
[firefox](https://www.mozilla.org/en-US/firefox/new/) v86.0 installed, and
`geckodriver` must be available on the PATH.

Additionally, you may want to use a VPN because Google's recapcha will
detect too many vaccination signups.  If you do end up getting flagged
by Google's backend, then this tool will no longer work and you may
not be able to signup on HEB's website using your IP address.

Once the above has been satisified, then run:

```bash
$ vax --auto
```

