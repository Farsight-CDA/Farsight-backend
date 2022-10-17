use std::str::FromStr;

use actix_web::web::Json;
use ethers::{abi::Address, types::U256};

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

    let from: Address = Address::from_str("0xf9E260cB9DfE2d57F3517822227F8041acB3E9D2").unwrap();
    let est = main_reg_cnt
        .receive_expiration_info(
            req.parameter.name,
            req.parameter.reg_version,
            req.parameter.owner_change_version,
            req.parameter.expiration,
        )
        .from(from)
        .estimate_gas()
        .await?;

    let add = get_config().bridge_base_gas;
    Ok(est + add)
}
