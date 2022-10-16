use ethers::{abi::Address, types::U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Request {
    pub name: String,
    pub expiry: U256,
    pub duration: U256,
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub token: Address,
    pub amount: U256,
}
