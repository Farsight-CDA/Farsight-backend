use ethers::{abi::Address, types::U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: U256,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub available: bool,
    pub chain_states: Vec<ChainState>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ChainState {
    #[serde(rename = "chainId")]
    pub chain_id: U256,
    pub owner: Address,
    pub expiration: U256,
    #[serde(rename = "isKeeper")]
    pub is_keeper: bool,
    pub owner_change_version: u64,
    pub registration_version: u64,
}
