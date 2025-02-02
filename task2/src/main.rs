use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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


struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}



impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
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

// struct Book{
//     title: String,
//     author: String,
//     year: u32,
// }
//
// struct BookStore{
//     books:HashMap<u32, Book>,
//     next_id: u32,
// }
//
// impl BookStore{
//     fn new()-> Self {
//         BookStore{
//             books: HashMap::new(),
//             next_id: 1,
//         }
//     }
//
//     fn get_books(&self) -> Vec<Book> {
//         self.books.values().cloned().collect()
//     }
//
//
//     fn get_book(&self, id:u32) -> Option<&Book>{
//         self.books.get(&id)
//     }
//
//     fn add_book(&mut self, book: Book) -> u32 {
//         let id = self.next_id;
//         self.books.insert(id, book);
//         self.next_id += 1;
//         id
//     }
//
//     fn update_book(&mut self, id:u32, book:Book) -> bool{
//         if self.books.contains_key(&id){
//             self.books.insert(id, book);
//             true
//         }else {
//             false
//         }
//     }
//
//     fn delete_book(&mut self, id:u32) -> bool{
//         self.books.remove(&id).is_some()
//     }
//
// }