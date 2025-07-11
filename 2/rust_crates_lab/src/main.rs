mod api;
mod models;

use actix_web::{web, App, HttpServer};
use api::search_news;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/search", web::get().to(search_news))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
