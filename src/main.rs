pub mod webserver;
pub mod config;

use actix_web;
use config::Config;
use once_cell::sync::OnceCell;

include!("../types_output/abi_types.rs");

pub const CONFIG_PATH: &str = "./config";
pub const CONFIG_FILE: &str = "config.toml";

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
    std::env::set_var("LOGLEVEL", "debug");
    pretty_env_logger::init_custom_env("LOGLEVEL");

    CONFIG.set(Config::load(CONFIG_PATH, CONFIG_FILE).expect("Failed to load config")).unwrap();

    webserver::run().await
}
