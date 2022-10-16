use crate::types::contract::ContractType;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use std::{slice::Iter, sync::Arc};

pub struct ProviderManager {
    provider: Vec<ProviderEntry>,
}

pub struct ProviderEntry {
    provider: Arc<Provider<Http>>,
    provider_url: String,
    addresses: Vec<ProviderAddress>,
    is_main: bool,
}

#[derive(Debug)]
pub struct ProviderAddress {
    address: Address,
    contract_type: ContractType,
}

impl ProviderAddress {
    pub fn new(address: Address, contract_type: ContractType) -> Self {
        Self {
            address,
            contract_type,
        }
    }

    pub fn contract_type(&self) -> ContractType {
        self.contract_type
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

impl ProviderEntry {
    pub fn new(
        provider: Provider<Http>,
        url: String,
        is_main: bool,
        addresses: Vec<ProviderAddress>,
    ) -> Self {
        Self {
            provider: Arc::new(provider),
            provider_url: url,
            is_main,
            addresses,
        }
    }

    pub fn provider_url(&self) -> &str {
        &self.provider_url
    }

    pub fn provider(&self) -> Arc<Provider<Http>> {
        self.provider.clone()
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn addresses(&self) -> &[ProviderAddress] {
        self.addresses.as_ref()
    }

    pub fn contract_address(&self, ct: ContractType) -> Option<&ProviderAddress> {
        self.addresses.iter().find(|i| i.contract_type == ct)
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

    pub fn has_main(&self) -> bool {
        self.provider.iter().find(|i| i.is_main).is_some()
    }

    pub fn provider_iter(&self) -> Iter<'_, ProviderEntry> {
        self.provider.iter()
    }

    pub fn get_main(&self) -> &ProviderEntry {
        self.provider
            .iter()
            .find(|i| i.is_main)
            .expect("No main provider configured")
    }
}
