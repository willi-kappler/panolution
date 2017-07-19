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

mod util;

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
        Ok((all_image_paths, all_thumbnail_paths)) => {
            // Run optimizer once for the smallest image size (scale factor):
            let mut current_arrangement = optimize(None, &config, &all_thumbnail_paths[0]);

            // Loop through all scale factors and find the optimal solution for each step
            // Use that result as input for the next scale factor iteration
            for thumbnail_path in all_thumbnail_paths.iter().skip(1) {
                current_arrangement = optimize(Some(&current_arrangement), &config, &thumbnail_path);
            }

            // Write result as big panorama image

            write_image(&current_arrangement, &config, all_image_paths);
        },
        Err(e) => {
            error!("An error occured: {}", e);

            for e in e.iter().skip(1) {
                error!("Caused by '{}'", e)
            }
        }
    }
}
