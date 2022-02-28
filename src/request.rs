use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Quote {
    pub content: String,
    pub author: String,
    pub length: u16,
    pub tags: Vec<String>,
}

pub async fn get_quote(url: String) -> Result<Quote, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let quote: Quote = response.json().await?;
    Ok(quote)
}
