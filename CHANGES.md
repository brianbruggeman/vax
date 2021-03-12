# Release History

## 0.5.0 (?)

- Adds native bundling for operating systems
- Adds option for filtering out appointments that are already taken
- Adds cargo-make

## 0.4.0 (2021-03-06)
Updates for Windows

- Spinners library fails to run on Windows so a build.rs file was added
  which checks for the Windows platform at build time and dynamically
  generates a new cfg, spin, which is then used to cut out spinners
  on Windows.
- Clippy warnings removed.
- Dead and commented out code removed.
- Improved error handling for missing address or missing lat/lon
- Reran formatting
- Upgraded packages
  
## 0.3.0 (2021-03-06)
Improvements to CLI usability

- Now runs in parallel
- Now Ignores unavailable signups
- Now handles reqwest errors rather than panicing
- Adds spinner and message so user can see something running
- Adds color to output


## 0.2.0 (2021-03-04)
Decrease CLI message spam

- Adds verbosity cli flag
- Reduces message output
- Adds clippy and fmt suggestions


## 0.1.0 (2021-02-11)
Initial release