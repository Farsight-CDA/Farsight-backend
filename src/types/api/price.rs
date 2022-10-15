use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub expiry: U256,
    pub duration: U256,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub token: String,
    pub amount: U256,
}
