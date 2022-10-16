use ethers::types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequestParam {
    pub plain_name: String,
    pub name: U256,
    pub owner: String,
    pub duration: U256,
    pub expiration: U256,
}
