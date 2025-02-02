use actix::{Actor, StreamHandler};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<IncomingMessage>(&text) {
                    Ok(parsed) => {
                        let response = match parsed.action.as_str() {
                            "get_books" => json!({ "status": "ok", "message": "get_books" }),
                            "get_book" => json!({ "status": "ok", "message": "get_book"  }),
                            "add_book"=> json!({"status": "ok", "message": "add_book" }),
                            "update_book"=> json!({"status": "ok", "message": "update_book" }),
                            "delete_book"=> json!({"status": "ok", "message": "delete_book" }),
                            _ => json!({ "status": "error", "message": "Unknown action" }),
                        };

                        ctx.text(response.to_string());
                    }
                    Err(_) => {
                        let error_response = json!({ "status": "error", "message": "Invalid JSON" });
                        ctx.text(error_response.to_string());
                    }
                }
            }
            _ => (),
        }
    }
}


#[get("/ws")]
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWs, &req, stream)
}

const MAX_FRAME_SIZE: usize = 16_384; // 16KiB

#[get("/custom-ws")]
async fn custom_websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::WsResponseBuilder::new(MyWs, &req, stream)
        .frame_size(MAX_FRAME_SIZE)
        .protocols(&["A", "B"])
        .start()
}

#[derive(Deserialize)]
struct IncomingMessage {
    action: String,
    data: Value,
}

#[derive(Serialize)]
struct ResponseMessage {
    status: String,
    message: String,
}

