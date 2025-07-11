use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenv::dotenv;
mod models;
use models::{SearchQuery, SearchResponse, Coin};
use actix_files::Files;
use reqwest;
use std::env;

// Fetch cryptocurrency data using the CoinGecko API
async fn fetch_articles(query: &str) -> Result<Vec<Article>, reqwest::Error> {
    let api_key = std::env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set in .env");

    let url = format!(
        "https://newsapi.org/v2/everything?q={}&apiKey={}",
        query, api_key
    );

    let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    let articles = response["articles"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|a| Article {
            title: a["title"].as_str().unwrap_or("").to_string(),
            description: a["description"].as_str().map(|s| s.to_string()),
            url: a["url"].as_str().unwrap_or("").to_string(),
        })
        .collect();

    Ok(articles)
} Ok(parsed.coins)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>Cryptocurrency Info App</title>
            <link rel="stylesheet" href="/static/style.css">
        </head>
        <body>
            <h1>Cryptocurrency Information</h1>
            <form action="/search" method="get">
                <input type="text" name="q" placeholder="Enter crypto (e.g., Bitcoin)">
                <button type="submit">Search</button>
            </form>
        </body>
        </html>
        "#,
    )
}

// Search for cryptocurrency information
async fn search_crypto(query: web::Query<SearchQuery>) -> impl Responder {
    let crypto_query = &query.q;

    let coins = fetch_articles(crypto_query).await.unwrap_or_default();

    let mut results_html = String::new();

    results_html.push_str("<h2>Cryptocurrency Information</h2>");
    if coins.is_empty() {
        results_html.push_str("<p>No cryptocurrencies found matching your search.</p>");
    } else {
        for coin in coins {
            let rank = coin.market_cap_rank.map_or(String::from("N/A"), |r| r.to_string());
            results_html.push_str(&format!(
                r#"
                <div class="coin">
                    <img src="{}" alt="{} logo">
                    <strong>{} ({})</strong> — Rank: {}
                </div>
                "#,
                coin.thumb, coin.name, coin.name, coin.symbol, rank
            ));
        }
    }

    HttpResponse::Ok().content_type("text/html").body(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>Cryptocurrency Info App</title>
            <link rel="stylesheet" href="/static/style.css">
        </head>
        <body>
            <h1>Search Results</h1>
            <p>You searched for: <strong>{}</strong></p>
            <div class="results">{}</div>
            <a href="/">← Back to Home</a>
        </body>
        </html>
        "#,
        crypto_query, results_html
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Get port from environment variable, with fallback to 8080
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    println!("Server running at http://localhost:{}/", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/search", web::get().to(search_crypto))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}