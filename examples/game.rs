use std::collections::HashMap;

use hex::ToHex;
use libpicacg::{api::{Auth, Host}, App, nonce, Header, Response, Responses, Client, Sort};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let mut client = Client::new();
    client.login("yinpeach", "20050314yjc.").await.unwrap();
    println!("profile: {:#?}", client.profile().await);
    let favorites = client.favorites(1, Sort::DescByDate).await;
    println!("favorites: {:#?}", favorites);
}
