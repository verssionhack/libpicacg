use std::collections::HashMap;

use hex::ToHex;
use libpicacg::{api::{auth, host}, app, nonce, Header, Response, responses, Sort};
use reqwest::Client;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let mut client = Client::new();
    client.login("", "").await.unwrap();
    println!("profile: {:#?}", client.profile().await);
    let favorites = client.favorites(1, Sort::DescByDate).await;
    println!("favorites: {:#?}", favorites);
}
