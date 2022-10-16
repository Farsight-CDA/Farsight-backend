use crate::{
    get_provider_manager,
    i_main_registrar::IMainRegistrar,
    types::{
        api::{error, registration},
        contract::ContractType,
    },
};
use crate::{DEFAULT_CACHE_SIZE, DEFAULT_CACHE_TIMEOUT};
use actix_web::web::Json;
use cached::proc_macro::cached;
use cached::TimedSizedCache;
use ethers::types::U256;

pub async fn handle(
    req: Json<registration::Request>,
) -> Result<Json<registration::Response>, error::Error> {
    if is_available(req.name).await? {
        return Ok(Json(registration::Response {
            available: true,
            chain_states: vec![],
        }));
    }

    todo!()
}

#[cached(
    type = "TimedSizedCache<U256, Result<bool, error::Error>>",
    create = "{ TimedSizedCache::with_size_and_lifespan(DEFAULT_CACHE_SIZE,DEFAULT_CACHE_TIMEOUT) }"
)]
async fn is_available(name: U256) -> Result<bool, error::Error> {
    //TimedSizedCache::with_size_and_lifespan
    let main_provider = get_provider_manager().get_main();

    let registrar = main_provider
        .contract_address(ContractType::Registrar)
        .unwrap();

    let main_reg = IMainRegistrar::new(registrar.address().clone(), main_provider.provider());
    Ok(main_reg.available(name).call().await?)
}
