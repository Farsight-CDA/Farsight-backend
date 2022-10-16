use actix_web::web::Json;
use ethers::types::U256;

use crate::{
    get_config, get_provider_manager,
    types::{
        api::{
            error,
            estimate::{self, expiration_info::ExpirationInfoParam, Request},
        },
        contract::ContractType,
    },
};

pub async fn handle(
    req: Json<Request<ExpirationInfoParam>>,
) -> Result<Json<estimate::Response>, error::Error> {
    let est = estimate(&req).await?;
    Ok(Json(estimate::Response { est }))
}

async fn estimate(req: &Request<ExpirationInfoParam>) -> Result<U256, error::Error> {
    let main_provider = get_provider_manager().main();
    let payment_address = main_provider
        .contract_address(ContractType::Controller)
        .unwrap();

    let main_reg_cnt = crate::IMainRegistrarController::new(
        payment_address.address().clone(),
        main_provider.provider(),
    );

    let est = main_reg_cnt
        .receive_expiration_info(
            req.parameter.name,
            req.parameter.reg_version,
            req.parameter.owner_change_version,
            req.parameter.expiration,
        )
        .estimate_gas()
        .await?;

    let add = get_config().bridge_base_gas;
    Ok(est + add)
}
