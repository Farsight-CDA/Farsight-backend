use actix_web::web::Json;
use ethers::types::U256;

use crate::{
    get_config, get_provider_manager,
    types::{
        api::{
            error,
            estimate::{self, renew::RenewRequestParam, Request},
        },
        contract::ContractType,
    },
};

pub async fn handle(
    req: Json<Request<RenewRequestParam>>,
) -> Result<Json<estimate::Response>, error::Error> {
    let est = estimate(&req).await?;
    Ok(Json(estimate::Response { est }))
}

async fn estimate(req: &Request<RenewRequestParam>) -> Result<U256, error::Error> {
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
        .receive_renew_request(
            chain_name.to_string(),
            req.parameter.name,
            req.parameter.reg_version,
            req.parameter.duration,
        )
        .estimate_gas()
        .await?;

    let add = get_config().bridge_base_gas;
    Ok(est + add)
}
