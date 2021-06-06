use actix_web::{Responder, HttpServer, HttpResponse, App, web,get};
use serde::{Serialize, Deserialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("I'm running ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(index)
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
