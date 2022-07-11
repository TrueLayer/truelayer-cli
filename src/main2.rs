use std::fmt::format;
use std::iter::Map;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpRequest};
use json;

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello")
}

#[post("/")]
async fn greet(req: HttpRequest, body: web::Bytes) -> impl Responder {


    let jsonstr = std::str::from_utf8(&body).unwrap();
    println!("{}", req.headers().get("tl-signature").unwrap().to_str().unwrap());
    println!("{}", jsonstr);
    // let result = json::parse(jsonstr); // return Result

    // println!("{}", result.unwrap());
   "ok"
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}