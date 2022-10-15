use ethers::types::U256;
use serde::{de, Deserializer};

pub fn deser_u256<'de, D>(data: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u64 = de::Deserialize::deserialize(data)?;
    Ok(U256::from(s))
}
