use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

use crate::{
    get_config,
    handlers::{get_price, get_registration, img_gen},
};

pub async fn run() -> std::io::Result<()> {
    let address = get_config().webserver.bind_address.as_str();
    let port = get_config().webserver.port;

    HttpServer::new(|| {
        App::new().service(index).service(
            web::scope("/api")
                .route("getPrice", web::post().to(get_price::handle))
                .route("getRegistration", web::post().to(get_registration::handle))
                .route("genImg", web::post().to(img_gen::handle)),
        )
    })
    .bind((address, port))?
    .run()
    .await
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Index page"
}
