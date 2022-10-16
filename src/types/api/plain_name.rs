use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Request {
    pub name: U256,
}

#[derive(Serialize, Clone)]
pub struct Response {
    #[serde(rename = "plainName")]
    pub plain_name: String,
}
