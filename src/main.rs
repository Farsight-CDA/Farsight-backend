pub mod config;
pub mod handlers;
pub mod provider_manager;
pub mod types;
pub mod webserver;

use std::str::FromStr;

use actix_web;
use config::Config;
use ethers::{
    abi::Address,
    providers::{Http, Provider},
};
use log::{debug, error};
use once_cell::sync::OnceCell;
use provider_manager::{ProviderAddress, ProviderEntry, ProviderManager};

include!("../types_output/abi_types.rs");

pub const CONFIG_PATH: &str = "./config";
pub const CONFIG_FILE: &str = "config.toml";

pub static CONFIG: OnceCell<Config> = OnceCell::new();
pub static PROVIDER: OnceCell<ProviderManager> = OnceCell::new();

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn get_provider_manager() -> &'static ProviderManager {
    PROVIDER.get().unwrap()
}

use ethers::prelude::*;

const LOGLEVEL: &str = "LOGLEVEL";

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    if std::env::var(LOGLEVEL).is_err() {
        std::env::set_var(LOGLEVEL, "debug");
    }
    pretty_env_logger::init_custom_env(LOGLEVEL);

    let config = Config::load(CONFIG_PATH, CONFIG_FILE).expect("Failed to load config");

    let manager = load_provider_manager(&config);
    if !manager.has_provider() {
        error!("No provider definied");
        std::process::exit(1);
    }
    PROVIDER.set(manager).ok().unwrap();

    CONFIG.set(config).unwrap();

    webserver::run().await
}

fn load_provider_manager(config: &Config) -> ProviderManager {
    let mut manager = ProviderManager::new();

    for p in &config.provider {
        let addresses = p
            .addresses
            .iter()
            .map(|pa| {
                let add: Address = Address::from_str(&pa.address).expect("Invalid address");
                ProviderAddress::new(add, pa.c_type)
            })
            .collect::<Vec<_>>();
        let provider = Provider::<Http>::try_from(&p.url).expect("Invalid provider url");
        debug!("Loading {} with {} addresse(s)", p.url, addresses.len());
        manager.add_provider(ProviderEntry::new(provider, p.is_main, addresses));
    }

    manager
}
