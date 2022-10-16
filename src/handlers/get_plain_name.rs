use crate::{
    get_provider_manager,
    types::{
        api::{error, plain_name},
        contract::ContractType,
    },
};
use crate::{DEFAULT_CACHE_SIZE, DEFAULT_CACHE_TIMEOUT};
use actix_web::web::Json;
use cached::proc_macro::cached;
use cached::TimedSizedCache;
use ethers::types::U256;

pub async fn handle(
    req: Json<plain_name::Request>,
) -> Result<Json<plain_name::Response>, error::Error> {
    Ok(Json(fetch_plain_name(req.name).await?))
}

#[cached(
    type = "TimedSizedCache<U256, Result<plain_name::Response, error::Error>>",
    create = "{ TimedSizedCache::with_size_and_lifespan(DEFAULT_CACHE_SIZE,DEFAULT_CACHE_TIMEOUT) }"
)]
async fn fetch_plain_name(name: U256) -> Result<plain_name::Response, error::Error> {
    let main_provider = get_provider_manager().get_main();
    let payment_address = main_provider
        .contract_address(ContractType::Registrar)
        .unwrap();

    let main_reg =
        crate::IMainRegistrar::new(payment_address.address().clone(), main_provider.provider());

    let plain_name = main_reg.lookup_plain_name(name).call().await?;
    Ok(plain_name::Response { plain_name })
}
