use crate::{
    get_provider_manager,
    i_main_registrar::IMainRegistrar,
    types::{
        api::{error, registration},
        contract::ContractType,
    },
};
use actix_web::web::Json;

pub async fn handle(
    req: Json<registration::Request>,
) -> Result<Json<registration::Response>, error::Error> {
    let main_provider = get_provider_manager().get_main();

    let registrar = main_provider
        .contract_address(ContractType::Registrar)
        .expect("Main provider doesn't have registrar set up");

    let main_reg = IMainRegistrar::new(registrar.address().clone(), main_provider.provider());

    if main_reg.available(req.name).call().await? {
        return Ok(Json(registration::Response {
            available: true,
            chain_states: vec![],
        }));
    }

    // owner = localOwner(req.name)
    todo!()
}
