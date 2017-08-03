// For error-chain:
#![recursion_limit = "1024"]

// External crates:
extern crate darwin_rs;
extern crate clap;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;
#[macro_use] extern crate error_chain;
extern crate itertools;
extern crate walkdir;
extern crate image;
extern crate imageproc;
extern crate rand;

// External imports:
use itertools::Itertools;


// Internal modules:
mod config;
use config::create_config;

mod logger;
use logger::create_logger;

mod error;

mod optimizer;
use optimizer::optimize;

mod output;
use output::write_image;

fn main() {
    // Init logger
    create_logger();

    let config = create_config();

    info!("Configuration option:");
    info!("input path: '{}'", config.input_path);
    info!("max iteration: '{}'", config.max_iteration);

    let mut solution = optimize(None, &config);

    write_image(&solution, &config);
}
