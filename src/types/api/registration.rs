use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    available: bool,
    chain_states: Vec<ChainState>,
}

#[derive(Serialize, Deserialize)]
pub struct ChainState {
    #[serde(rename = "chainId")]
    chain_id: U256,
    owner: String,
    expiration: U256,
    #[serde(rename = "isKeeper")]
    is_keeper: bool,
}
