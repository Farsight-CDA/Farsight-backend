use actix_web::{web::Json, get};
use crate::types::api::{price, error};

#[get("/api/getPrice")]
async fn handle(req: Json<price::Request>) -> Result<Json<price::Response>, error::Error> {
    todo!()
}
