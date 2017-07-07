// External modules
use clap::{Arg, App};
use toml;

// System modules
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
//use std::cmp;

// Internal modules
use error::{Error, ErrorKind, Result, ResultExt};

#[derive(Clone, Debug, PartialEq)]
pub struct Configuration {
    pub input_path: String,
    pub max_iteration: u64,
    pub scale_factors: Vec<f64>,
}

#[derive(Deserialize)]
struct TOMLConfig {
    input_path: Option<String>,
    max_iteration: Option<u64>,
}

fn default_config() -> Configuration {
    Configuration {
        input_path: "./".to_string(),
        max_iteration: 10000,
        scale_factors: vec![0.1, 0.3],
    }
}

pub fn create_config() -> Configuration {
    let version = "0.1";

    let matches = App::new("Panolution")
        .version(version).author("Willi Kappler <grandor@gmx.de>").about("Panorama photo stitcher written in Rust")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a configuration file, command line arguments can overwrite values from the config file")
            .takes_value(true))
        .arg(Arg::with_name("max_iteration")
            .short("m")
            .long("max-iteration")
            .value_name("MAX_ITERATION")
            .help("Sets the maximum number of iteration for each size step")
            .takes_value(true))
        .arg(Arg::with_name("scale_factors")
            .short("s")
            .long("scale-factors")
            .value_name("SCALE_FACTORS")
            .help("Sets the scale factor to use for arranging images")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .help("Sets the input file or folder, default is current folder './'")
            .index(1))
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Shows version number"))
        .after_help(
            "Examples:\n\
             # input path: current folder './'\n\
             panolution\n\n\
            "
        )
        .get_matches();

    // Default values:
    let mut result = default_config();

    if let Some(config_file) = matches.value_of("config") {
        match load_config(config_file) {
            Ok(config) => {
                info!("Configuration successfully loaded")
            },
            Err(Error(ErrorKind::IOOpenError, _)) => {
                warn!("Could not open configuration file '{}'", config_file)
            },
            Err(Error(ErrorKind::IOReadError, _)) => {
                warn!("Could not read configuration file '{}'", config_file)
            },
            Err(Error(ErrorKind::TOMLError, _)) => {
                warn!("Could not parse configuration file '{}', TOML error", config_file)
            },
            Err(Error(ErrorKind::Msg(m), _)) => {
                warn!("Some other error occured: {}", m)
            },
        }
    }

    // Command line parameter can overwrite configuration file settings
    if let Some(input_path) = matches.value_of("input") {
        result.input_path = input_path.to_string();
        info!("input path: {}", input_path);
    }

    result
}

fn load_config(filename: &str) -> Result<TOMLConfig> {
    info!("Loading configuration file: {}", filename);

    let file = OpenOptions::new().read(true).open(filename).chain_err(|| ErrorKind::IOOpenError)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content).chain_err(|| ErrorKind::IOReadError)?;
    toml::from_str::<TOMLConfig>(&content).chain_err(|| ErrorKind::TOMLError)
}

#[cfg(test)]
mod test {
    use super::{default_config, Configuration};

    use logger::create_logger;

}
