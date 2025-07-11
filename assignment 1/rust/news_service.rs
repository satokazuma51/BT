// src/services/news_service.rs
use serde::Deserialize;
use reqwest;
use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
}

pub struct NewsService {
    api_key: String,
}

impl NewsService {
    pub fn new(config: Config) -> Self {
        Self {
            api_key: config.news_api_key,
        }
    }

    pub async fn fetch_news(&self, keyword: &str) -> Result<Vec<NewsArticle>, reqwest::Error> {
        let url = format!(
            "https://newsapi.org/v2/everything?q={}&apiKey={}",
            keyword, self.api_key
        );

        let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

        // Мапим JSON вручную или через строго типизированную структуру
        let articles = response["articles"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|a| NewsArticle {
                title: a["title"].as_str().unwrap_or("").to_string(),
                description: a["description"].as_str().map(|d| d.to_string()),
                url: a["url"].as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(articles)
    }
}
