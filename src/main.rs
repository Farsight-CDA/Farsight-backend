pub mod config;
pub mod handlers;
pub mod provider_manager;
pub mod types;
pub mod webserver;

use std::{str::FromStr, sync::Arc};

use actix_web;
use config::Config;
use ethers::{
    abi::Address,
    providers::{Http, Provider},
};
use log::{debug, error};
use once_cell::sync::OnceCell;
use provider_manager::{ProviderAddress, ProviderEntry, ProviderManager};
use types::contract::ContractType;

include!("../types_output/abi_types.rs");

const LOGLEVEL: &str = "LOGLEVEL";

pub const CONFIG_PATH: &str = "./config";
pub const CONFIG_FILE: &str = "config.toml";

pub static CONFIG: OnceCell<Config> = OnceCell::new();
pub static PROVIDER: OnceCell<ProviderManager> = OnceCell::new();

pub const DEFAULT_CACHE_TIMEOUT: u64 = 3 * 60; // 3 Min timeout
pub const DEFAULT_CACHE_SIZE: usize = 1_000; // Each cached function has a limit of 1000

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn get_provider_manager() -> &'static ProviderManager {
    PROVIDER.get().unwrap()
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    if std::env::var(LOGLEVEL).is_err() {
        std::env::set_var(LOGLEVEL, "debug");
    }
    pretty_env_logger::init_custom_env(LOGLEVEL);

    let config = Config::load(CONFIG_PATH, CONFIG_FILE).expect("Failed to load config");

    let manager = load_provider_manager(&config).await;
    if let Err(err) = check_provider_manager(&manager) {
        error!("Provider config invalid: {err}");
        std::process::exit(1);
    }
    PROVIDER.set(manager).ok().unwrap();

    CONFIG.set(config).unwrap();

    webserver::run().await
}

async fn load_provider_manager(config: &Config) -> ProviderManager {
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

        let registrar_address = addresses
            .iter()
            .find(|i| i.contract_type() == ContractType::Registrar)
            .expect(&format!("Registrar address not set up for {}", p.url));
        let registrar = IMainRegistrar::new(
            registrar_address.address().clone(),
            Arc::new(provider.clone()),
        );
        let bridge_address = registrar
            .get_name_bridge()
            .call()
            .await
            .expect(&format!("Failed to get bridge for {}", p.url));

        debug!("Loading {} with {} addresse(s)", p.url, addresses.len());

        manager.add_provider(ProviderEntry::new(
            provider,
            p.url.clone(),
            p.name.clone(),
            p.id,
            p.is_main,
            bridge_address,
            addresses,
        ));
    }

    manager
}

fn check_provider_manager(pm: &ProviderManager) -> Result<(), String> {
    if !pm.has_provider() {
        return Err(String::from("No provider defined"));
    }

    if !pm.has_main() {
        return Err(String::from("No main provider defined"));
    }

    // Assert main provider having all contract types setup correctly
    let main = pm.main();
    for ctype in ContractType::iter() {
        if main.contract_address(ctype).is_none() {
            return Err(format!(
                "Main provider doesn't have Contract type: {ctype:?}"
            ));
        }
    }

    // Assert all providers having `Registrar` as `ContractType`
    for pv in pm.provider_iter() {
        if pv.contract_address(ContractType::Registrar).is_none() {
            return Err(format!(
                "Provider {} doesn't have a registrar setup",
                pv.provider_url()
            ));
        }
    }

    Ok(())
}
