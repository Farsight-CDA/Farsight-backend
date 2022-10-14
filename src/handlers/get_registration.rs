use actix_web::{web::Json, get, Responder};
use crate::types::api::{self, registration, error};

#[get("/api/getPrice")]
async fn handle(req: Json<registration::Request>) -> Result<Json<registration::Response>, error::Error> {
    todo!()
}
