use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello")
}

#[post("/")]
pub async fn index(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}