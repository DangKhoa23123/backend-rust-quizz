use actix_web::{App, HttpServer};
mod model;
mod router;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server chạy tại http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .configure(config::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
