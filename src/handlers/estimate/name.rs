use actix_web::web::Json;
use ethers::types::U256;

use crate::{
    get_provider_manager,
    types::{
        api::{
            error,
            estimate::{self, name::NameParam, Request},
        },
        contract::ContractType,
    },
};

pub async fn handle(
    req: Json<Request<NameParam>>,
) -> Result<Json<estimate::Response>, error::Error> {
    let est = estimate(&req).await?;
    Ok(Json(estimate::Response { est }))
}

async fn estimate(req: &Request<NameParam>) -> Result<U256, error::Error> {
    let main_provider = get_provider_manager().main();
    let payment_address = main_provider
        .contract_address(ContractType::Controller)
        .unwrap();

    let main_reg_cnt = crate::IMainRegistrarController::new(
        payment_address.address().clone(),
        main_provider.provider(),
    );

    let est = main_reg_cnt
        .receive_name(
            req.parameter.name,
            req.parameter.reg_version,
            req.parameter.owner_change_version,
            req.parameter.expiration,
            req.parameter.owner,
        )
        .estimate_gas()
        .await?;
    Ok(est)
}
