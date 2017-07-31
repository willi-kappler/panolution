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

mod thumbnail;
use thumbnail::create_thumbnails;

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
    info!("scale_factors: '{}'", config.scale_factors.iter().join(", "));

    // Create thumbnails if necessary:
    match create_thumbnails(&config) {
        Ok(solutions) => {
            // Run optimizer once for the smallest image size (scale factor):
            let mut new_solution = optimize(&solutions[0], &config);

            // Loop through all scale factors and find the optimal solution for each step
            // Use that result as input for the next scale factor iteration
            for solution in solutions.iter().skip(1) {
                new_solution.update_path_from(&solution);
                new_solution = optimize(&new_solution, &config);
            }

            // Write result as big panorama image
            write_image(&new_solution, &config);
        },
        Err(e) => {
            error!("An error occured: {}", e);

            for e in e.iter().skip(1) {
                error!("Caused by '{}'", e)
            }
        }
    }
}
