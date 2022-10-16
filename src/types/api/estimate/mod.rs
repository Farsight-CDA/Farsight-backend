pub mod expiration_info;
pub mod name;
pub mod register;
pub mod renew;

use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request<T> {
    #[serde(default)]
    pub chain_id: String,

    #[serde(flatten)]
    pub parameter: T,
}

#[derive(Serialize)]
pub struct Response {
    pub est: U256,
}
