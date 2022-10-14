use actix_web::{get,App, HttpServer, HttpRequest, Responder};

use crate::{get_config, handlers::{get_price, get_registration}};

pub async fn run() -> std::io::Result<()>{
    let address = get_config().webserver.bind_address.as_str();
    let port = get_config().webserver.port;

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_price::handle)
            .service(get_registration::handle)
    })
    .bind((address, port))?
    .run()
    .await
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}
