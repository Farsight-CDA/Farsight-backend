use ethers::types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExpirationInfoParam {
    pub name: U256,
    pub reg_version: u64,
    pub owner_change_version: u64,
    pub expiration: U256,
}
