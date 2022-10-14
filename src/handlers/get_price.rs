use crate::types::api::{error, price};
use actix_web::web::Json;

pub async fn handle(req: Json<price::Request>) -> Result<Json<price::Response>, error::Error> {
    Err(error::Error::Internal)
}
