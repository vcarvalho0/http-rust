pub mod routes;

use std::io;
use actix_web::{HttpServer, App};
use routes::{hello, index};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(index)
    })
    .bind(("0.0.0.0", 8080))
    ?.run()
    .await
}