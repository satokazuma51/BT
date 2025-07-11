// models.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Deserialize)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub thumb: String,
    pub market_cap_rank: Option<u32>,
}

#[derive(Deserialize)]
pub struct SearchResponse {
    pub coins: Vec<Coin>,
}
