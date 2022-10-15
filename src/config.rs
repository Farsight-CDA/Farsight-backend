use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::types::contract::ContractType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub provider: Vec<Provider>,
    pub webserver: Webserver,
    pub image: Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub url: String,
    pub is_main: bool,
    pub addresses: Vec<ProviderAddress>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderAddress {
    pub address: String,
    pub c_type: ContractType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webserver {
    pub bind_address: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub python_bin: String,
    pub image_cache: String,
    pub bg_image: String,
    pub font: String,
}

impl Default for Webserver {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            port: 8081,
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            python_bin: "/usr/bin/python".to_string(),
            image_cache: "./img_cache".to_string(),
            bg_image: "./background.png".to_string(),
            font: "./SourceCodePro-Bold.ttf".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            webserver: Default::default(),
            image: Image::default(),
            provider: vec![Provider {
                url: "http://url".to_string(),
                addresses: vec![ProviderAddress {
                    address: "1234".to_string(),
                    c_type: ContractType::Registrar,
                }],
                is_main: true,
            }],
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
