use std::sync::Arc;

use ethers::{
    abi::Address,
    providers::{Http, Provider},
};

use crate::types::contract::ContractType;

pub struct ProviderManager {
    provider: Vec<ProviderEntry>,
}

pub struct ProviderEntry {
    provider: Provider<Http>,
    addresses: Vec<ProviderAddress>,
    is_main: bool,
}

#[derive(Debug)]
pub struct ProviderAddress {
    address: Arc<Address>,
    contract_type: ContractType,
}

impl ProviderAddress {
    pub fn new(address: Address, contract_type: ContractType) -> Self {
        Self {
            address: Arc::new(address),
            contract_type,
        }
    }
}

impl ProviderEntry {
    pub fn new(provider: Provider<Http>, is_main: bool, addresses: Vec<ProviderAddress>) -> Self {
        Self {
            provider,
            is_main,
            addresses,
        }
    }

    pub fn provider(&self) -> &Provider<Http> {
        &self.provider
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }
}

impl ProviderManager {
    pub fn new() -> Self {
        Self { provider: vec![] }
    }

    pub fn add_provider(&mut self, provider: ProviderEntry) {
        self.provider.push(provider);
    }

    pub fn has_provider(&self) -> bool {
        !self.provider.is_empty()
    }
}
