use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    name: String,
    expiry: U256,
    duration: U256,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    token: String,
    amount: U256,
}
