use crate::{
    get_provider_manager,
    types::{
        api::{error, price},
        contract::ContractType,
    },
};
use actix_web::web::Json;

pub async fn handle(req: Json<price::Request>) -> Result<Json<price::Response>, error::Error> {
    let main_provider = get_provider_manager().get_main();
    let payment_address = main_provider
        .contract_address(ContractType::PaymentProvider)
        .expect("Main provider should have payment provider");

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

    let response = price::Response { token, amount };
    Ok(Json(response))
}
