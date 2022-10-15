use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ContractType {
    Controller,
    Registrar,
    PaymentProvider,
}
