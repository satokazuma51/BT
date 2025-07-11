// src/config.rs
#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub news_api_key: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Self {
            port: std::env::var("PORT").unwrap_or_else(|_| "8080".into()).parse().unwrap(),
            news_api_key: std::env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set"),
        }
    }
}
