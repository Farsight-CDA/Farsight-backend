use crate::{
    get_provider_manager,
    types::{
        api::{error, price},
        contract::ContractType,
    },
};
use crate::{DEFAULT_CACHE_SIZE, DEFAULT_CACHE_TIMEOUT};
use actix_web::web::Json;
use cached::proc_macro::cached;
use cached::TimedSizedCache;

pub async fn handle(req: Json<price::Request>) -> Result<Json<price::Response>, error::Error> {
    Ok(Json(fetch_data(&req).await?))
}

#[cached(
    type = "TimedSizedCache<price::Request, Result<price::Response, error::Error>>",
    create = "{ TimedSizedCache::with_size_and_lifespan(DEFAULT_CACHE_SIZE,DEFAULT_CACHE_TIMEOUT) }"
)]
async fn fetch_data(req: &price::Request) -> Result<price::Response, error::Error> {
    let main_provider = get_provider_manager().main();
    let payment_address = main_provider
        .contract_address(ContractType::PaymentProvider)
        .unwrap();

    let payment_provider = crate::IERC20PaymentProvider::new(
        payment_address.address().clone(),
        main_provider.provider(),
    );

    let name = &req.name;
    let expires = req.expiry;
    let duration = req.duration;
    let amount = payment_provider
        .get_price(name.clone(), expires, duration)
        .call()
        .await?;

    let token = payment_provider.get_token_address().call().await?;
    Ok(price::Response { token, amount })
}
