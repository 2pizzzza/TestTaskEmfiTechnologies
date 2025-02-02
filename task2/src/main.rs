mod api;

use actix::{Actor, StreamHandler};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use api::{custom_websocket, hello, echo, websocket};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(echo)
            .service(websocket)
            .service(custom_websocket)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



