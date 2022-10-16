use std::array::IntoIter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    Controller,
    Registrar,
    PaymentProvider,
}

impl ContractType {
    pub fn iter() -> IntoIter<ContractType, 3> {
        [
            ContractType::Controller,
            ContractType::Registrar,
            ContractType::PaymentProvider,
        ]
        .into_iter()
    }
}
