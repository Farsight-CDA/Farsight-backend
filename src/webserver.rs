use actix_web::{get,App, HttpServer, web, HttpRequest, Responder};

pub async fn run() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(index)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}
