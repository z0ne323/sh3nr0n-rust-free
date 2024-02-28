![dragon-ball-z-shenron](https://github.com/z0ne323/sh3nr0n-rust-free/assets/80288433/fd1e21a6-ddeb-49b8-bc8a-cca8e3b87900)

# sh3nr0n-rust-free ?

This project is intented to be a simple implementation of the free side of Shodan Rest API in Rust.

It's not at all intended to be something exhaustive with everything from the free part of Shodan API. If you see something missing, or not done the *proper* way (still learning Rust tbh) and would like me to add something or if you'd like to contribute, feel free to open a PR or an issue request!

# What's in there ?

You'll find different things in this repo:
- `cargo.lock` & `cargo.toml`, necessary configuration file no matter what you're doing with this project...
- `src/` folder contains various files:
    - `main.rs` -> Like the name state, main file of the project, calling functions and handling some logic
    - `helpers.rs` -> Helpers file, storing generic functions (so far only error handling)
    - `shodan.rs` -> probably most important file of this project, storing all the functions interacting with Shodan API and called in main !

# Links ?

## If you'd like... 
- To take a tour of all services proposed by Shodan (API and more): https://account.shodan.io/billing/tour
- To check the InternetDB API: https://internetdb.shodan.io/
- To check the GeoNet tools: https://geonet.shodan.io/
- To check the EntityDB API: https://entitydb.shodan.io/
