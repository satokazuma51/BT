use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub url: String,
}

pub async fn fetch_news(query: &str) -> Result<Vec<NewsArticle>, Box<dyn std::error::Error>> {
    let api_key = std::env::var("CRYPTO_NEWS_API_KEY")?;
    let url = format!("https://cryptonews-api.com/api/v1/category?section=general&items=10&extra-fields=summary&token={}", api_key);

    let res = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
    let mut results = vec![];

    if let Some(articles) = res.get("data").and_then(|d| d.as_array()) {
        for article in articles.iter() {
            let title = article.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let source = article.get("source_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let date = article.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let summary = article.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = article.get("news_url").and_then(|v| v.as_str()).unwrap_or("").to_string();

            if title.to_lowercase().contains(&query.to_lowercase()) || summary.to_lowercase().contains(&query.to_lowercase()) {
                results.push(NewsArticle { title, source, date, summary, url });
            }
        }
    }

    Ok(results)
}
