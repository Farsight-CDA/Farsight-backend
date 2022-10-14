use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub webserver: Webserver,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webserver {
    pub bind_address: String,
    pub port: u16,
}

impl Default for Webserver {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 8081,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            webserver: Default::default(),
        }
    }
}

impl Config {
    pub fn load(file_path: &str, file_name: &str) -> Result<Config, Box<dyn Error>> {
        let path = Path::new(file_path).join(file_name);

        if !path.exists() {
            log::debug!("Creating new config");

            let config = Config::default();
            let data = toml::to_vec(&config)?;

            if !Path::new(file_path).exists() {
                std::fs::create_dir_all(file_path)?;
            }

            let file = File::create(path)?;
            BufWriter::new(file).write(&data)?;
            return Ok(config);
        }

        log::debug!("Loading config from {}", file_path);
        let config_data = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config_data)?)
    }
}
