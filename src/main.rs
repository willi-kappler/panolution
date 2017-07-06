extern crate darwin_rs;
extern crate clap;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate chrono;

// use darwin_rs::{Individual, SimulationBuilder, Population, PopulationBuilder};


// Internal modules
mod config;
use config::create_config;

mod logger;
use logger::create_logger;

fn main() {
    // Init logger
    create_logger();

    let config = create_config();
}
