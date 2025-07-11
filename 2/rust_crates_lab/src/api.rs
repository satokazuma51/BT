use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::models::{NewsArticle, fetch_news};

#[derive(Deserialize)]
pub struct Query {
    pub query: String,
}

pub async fn search_news(query: web::Query<Query>) -> impl Responder {
    match fetch_news(&query.query).await {
        Ok(news) => HttpResponse::Ok().json(news),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
