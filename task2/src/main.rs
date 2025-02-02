mod service;
mod domain;
mod infrastructure;
mod api;

use crate::infrastructure::book_repository::Storage;
use crate::service::book_service::BookService;
use actix_web::{middleware, web, App, HttpServer};
use api::book_controller::websocket;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=info");


    let storage = Arc::new(Mutex::new(Storage::new()));
    let service = Arc::new(BookService::new(storage));
    let service_data = web::Data::new(service.clone());


    println!("The server is running on port 8080");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(service_data.clone())
            .service(websocket)
    })

        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


