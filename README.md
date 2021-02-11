# Vax

A COVD-19 Signup tool for use with H.E.B's Backend API.

## Quickstart

To run:

```bash
$ cargo run --release
```

## Options

The script has multiple command-line options:

```
vax 0.1.0
A Covid-19 Vaccination Signup Tool

USAGE:
    vax [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>        Physical address of individual needing a vaccine [env: ADDRESS=]  [default: ]
    -c, --carrier <carrier>        Phone carrier [env: CARRIER=]  [default: ]
    -e, --email <email>            Email address of program user [env: EMAIL=]  [default: ]
        --latitude <latitude>      Latitude of the physical address for individual needing a vaccine [env:
                                   LATITUDE=]  [default: 0.0]
        --longitude <longitude>    Longitude of the physical address for individual needing a vaccine [env:
                                   LONGITUDE=]  [default: 0.0]
    -p, --phone <phone>            Phone number of individual needing a vaccine [env: PHONE=]  [default: ]
    -t, --threshold <threshold>    Threshold in miles for how far to travel to get vaccine [env: THRESHOLD=25]
                                   [default: 25]
    -t, --timeout <timeout>        How often to ping H.E.B's API in milliseconds [env: TIMEOUT=1000]  [default: 10000]
```

This script can be controlled with environment variables or a dotenv file
specified by `DOTENV_FILE` or defaulting to `.env`:

```
# these are also all command-line options
ADDRESS="your address"
# CARRIER="your carrier"
# LATITUDE="your home coordinate latitude if you have it"
# LONGITUDE="your home coordinate longitude if you have it"
# EMAIL="your email"
# PHONE="your phone number"
THRESHOLD="25"
TIMEOUT="1000"
```


