use std::sync::Arc;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use serde::{Deserialize};
use serde_json::{json, Value};
use actix_web::{get, web, HttpResponse, HttpRequest, Error};
use crate::domain::book::Book;
use crate::infrastructure::book_repository::Storage;
use crate::service::book_service::BookService;

struct Ws {
    service: Arc<BookService<Storage>>,
}

impl Ws {
    fn new(service: Arc<BookService<Storage>>) -> Self {
        Self { service }
    }
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<IncomingMessage>(&text) {
                    Ok(parsed) => {
                        let response = match parsed.action.as_str() {
                            "get_books" => {
                                let books = self.service.get_books();
                                json!({ "status": "ok", "books": books })
                            }
                            "get_book" => {
                                if let Some(id) = parsed.data.get("id").and_then(|v| v.as_u64()) {
                                    match self.service.get_book(id as u32) {
                                        Some(book) => json!({ "status": "ok", "book": book }),
                                        None => json!({ "status": "error", "message": "Book not found" }),
                                    }
                                } else {
                                    json!({ "status": "error", "message": "Missing book ID" })
                                }
                            }
                            "add_book" => {
                                if let Some(book) = serde_json::from_value::<Book>(parsed.data.clone()).ok() {
                                    let id = self.service.add_book(book);
                                    json!({ "status": "ok", "id": id })
                                } else {
                                    json!({ "status": "error", "message": "Invalid book data" })
                                }
                            }
                            "update_book" => {
                                if let (Some(id), Some(book)) = (
                                    parsed.data.get("id").and_then(|v| v.as_u64()),
                                    serde_json::from_value::<Book>(parsed.data.clone()).ok(),
                                ) {
                                    if self.service.update_book(id as u32, book) {
                                        json!({ "status": "ok" })
                                    } else {
                                        json!({ "status": "error", "message": "Book not found" })
                                    }
                                } else {
                                    json!({ "status": "error", "message": "Invalid data" })
                                }
                            }
                            "delete_book" => {
                                if let Some(id) = parsed.data.get("id").and_then(|v| v.as_u64()) {
                                    if self.service.delete_book(id as u32) {
                                        json!({ "status": "ok" })
                                    } else {
                                        json!({ "status": "error", "message": "Book not found" })
                                    }
                                } else {
                                    json!({ "status": "error", "message": "Missing book ID" })
                                }
                            }
                            _ => json!({ "status": "error", "message": "Unknown action" }),
                        };

                        ctx.text(response.to_string());
                    }
                    Err(_) => {
                        ctx.text(json!({ "status": "error", "message": "Invalid JSON" }).to_string());
                    }
                }
            }
            _ => (),
        }
    }
}


#[get("/ws")]
pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    service: web::Data<Arc<BookService<Storage>>>,
) -> Result<HttpResponse, Error> {
    ws::start(Ws::new(Arc::clone(&service)), &req, stream)
}


#[derive(Deserialize)]
struct IncomingMessage {
    action: String,
    data: Value,
}



