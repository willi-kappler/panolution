// External modules
use clap::{Arg, App};
use toml;

// System modules
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
//use std::cmp;

#[derive(Clone, Debug, PartialEq)]
pub struct Configuration {
    pub input_path: String,

}

#[derive(Deserialize)]
struct TOMLConfig {
    input_path: Option<String>
}

fn default_config() -> Configuration {
    Configuration {
        input_path: "./".to_string(),
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
        result = load_config(config_file);
    }

    // Command line parameter can overwrite configuration file settings
    if let Some(input_path) = matches.value_of("input") {
        result.input_path = input_path.to_string();
        info!("input path: {}", input_path);
    }

    result
}

fn load_config(filename: &str) -> Configuration {
    info!("Loading configuration file: {}", filename);

    if let Ok(file) = OpenOptions::new().read(true).open(filename) {
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();

        if let Ok(_) = buf_reader.read_to_string(&mut content) {
            return parse_config_file(&content);
        } else {
            // Content of configuration file could not be read, use default settings instead.
            warn!("Could not read configuration file '{}', using default settings", filename);
        }
    } else {
        // Configuration file could not be opened, use default settings instead.
        warn!("Could not open configuration file '{}', using default settings", filename);
    }
    default_config()
}

fn parse_config_file(content: &str) -> Configuration {
    let mut result = default_config();

    match toml::from_str::<TOMLConfig>(content) {
        Ok(config) => {
            if let Some(input_path) = config.input_path {
                result.input_path = input_path;
            }
        },
        Err(e) => {
            warn!("TOML parse error: {}", e);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::{parse_config_file, default_config, Configuration};

    use logger::create_logger;

    #[test]
    fn parse_config_file1() {
        create_logger();

        let input = "";

        let expected_output = default_config();

        assert_eq!(parse_config_file(input), expected_output);
    }

    #[test]
    fn parse_config_file2() {
        create_logger();

        let input =
r#"
input_path = "scans"
"#;

        let expected_output = Configuration{
            input_path: "scans".to_string(),
        };

        assert_eq!(parse_config_file(input), expected_output);
    }
}
