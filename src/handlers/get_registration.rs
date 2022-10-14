use crate::types::api::{error, registration};
use actix_web::web::Json;

pub async fn handle(
    req: Json<registration::Request>,
) -> Result<Json<registration::Response>, error::Error> {
    todo!()
}
