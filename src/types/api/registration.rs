use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub available: bool,
    pub chain_states: Vec<ChainState>,
}

#[derive(Serialize, Deserialize)]
pub struct ChainState {
    #[serde(rename = "chainId")]
    pub chain_id: U256,
    pub owner: String,
    pub expiration: U256,
    #[serde(rename = "isKeeper")]
    pub is_keeper: bool,
}
