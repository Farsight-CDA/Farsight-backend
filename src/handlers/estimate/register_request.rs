use actix_web::web::Json;
use ethers::types::U256;

use crate::{
    get_provider_manager,
    types::{
        api::{
            error,
            estimate::{self, register::RegisterRequestParam, Request},
        },
        contract::ContractType,
    },
};

pub async fn handle(
    req: Json<Request<RegisterRequestParam>>,
) -> Result<Json<estimate::Response>, error::Error> {
    let est = estimate(&req).await?;
    Ok(Json(estimate::Response { est }))
}

async fn estimate(req: &Request<RegisterRequestParam>) -> Result<U256, error::Error> {
    let main_provider = get_provider_manager().main();
    let payment_address = main_provider
        .contract_address(ContractType::Controller)
        .unwrap();

    let main_reg_cnt = crate::IMainRegistrarController::new(
        payment_address.address().clone(),
        main_provider.provider(),
    );

    let chain_name = get_provider_manager()
        .by_id(req.chain_id as u64)
        .ok_or(error::Error::NotFound)?
        .name();

    let est = main_reg_cnt
        .receive_register_request(
            chain_name.to_string(),
            req.parameter.plain_name.clone(),
            req.parameter.name,
            req.parameter.owner.clone(),
            req.parameter.duration,
            req.parameter.expiration,
        )
        .estimate_gas()
        .await?;
    Ok(est)
}
