use ethers::types::U256;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenewRequestParam {
    pub name: U256,
    pub reg_version: u64,
    pub duration: U256,
}
