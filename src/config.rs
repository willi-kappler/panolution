// External modules
use clap::{Arg, App, ArgMatches};
use toml;

// System modules
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
//use std::cmp;

// Internal modules
use error::{Result, ResultExt};

macro_rules! assign_if{
    ($left:expr, $right:expr) => {
        if let Some(value) = $right {
            $left = value;
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanolutionConfig {
    pub input_path: String,
    pub max_iteration: u32,
    pub num_of_threads: usize,
    pub num_of_samples: Vec<u32>,
}

#[derive(Deserialize)]
struct TOMLConfig {
    input_path: Option<String>,
    max_iteration: Option<u32>,
    _num_of_threads: Option<usize>,
    _num_of_samples: Option<Vec<u32>>,
}

fn default_config() -> PanolutionConfig {
    PanolutionConfig {
        input_path: "./".to_string(),
        max_iteration: 1000,
        num_of_threads: 4,
        num_of_samples: vec![500, 2000, 5000],
    }
}

pub fn create_config() -> PanolutionConfig {
    let version = "0.1";

    let matches = App::new("Panolution")
        .version(version).author("Willi Kappler <grandor@gmx.de>").about("Panorama photo stitcher written in Rust")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a PanolutionConfig file, command line arguments can overwrite values from the config file")
            .takes_value(true))
        .arg(Arg::with_name("max_iteration")
            .short("m")
            .long("max-iteration")
            .value_name("MAX_ITERATION")
            .help("Sets the maximum number of iteration for each size step")
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

    match process_config(matches) {
        Ok(config) => {
            config
        },
        Err(e) => {
            warn!("An error occured: '{}'", e);

            for e in e.iter().skip(1) {
                warn!("Caused by '{}'", e)
            }

            info!("Using default configuration");

            default_config()
        }
    }
}

fn process_config(matches: ArgMatches) -> Result<PanolutionConfig> {
    // Default values, these will be overwritten if needed:
    let mut result = default_config();

    if let Some(config_file) = matches.value_of("config") {
        let toml_config = load_config(config_file)?;

        assign_if!(result.input_path, toml_config.input_path);
        assign_if!(result.max_iteration, toml_config.max_iteration);
    }

    // Command line parameter can overwrite PanolutionConfig file settings:

    if let Some(input_path) = matches.value_of("input") {
        result.input_path = input_path.to_string();
    }

    if let Some(max_iteration) = matches.value_of("max_iteration") {
        result.max_iteration = max_iteration.parse::<u32>().chain_err(|| format!("can't parse command line integer value: '{}'", max_iteration))?;
    }

    Ok(result)
}

fn load_config(file_name: &str) -> Result<TOMLConfig> {
    info!("Loading PanolutionConfig file: {}", file_name);

    let file = OpenOptions::new().read(true).open(file_name).chain_err(|| format!("can't open file: '{}'", file_name))?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content).chain_err(|| "can't read to buffer")?;
    toml::from_str::<TOMLConfig>(&content).chain_err(|| "can't parse TOML file")
}
