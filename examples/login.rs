use std::collections::HashMap;

use hex::ToHex;
use libpicacg::{api::{auth, host}, app, nonce, Header, Response, responses::{self, Comments, ComicChildrenComment}, Api, Sort, ApiResult};
use reqwest::Proxy;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let mut api = Api::new();
    api.set_proxy(Some(Proxy::all("http://localhost:15777").unwrap())).unwrap();
    api.login("yinpeach", "20050314yjc.").await.unwrap();
    let mut v = api.favorites(1, Sort::DescByDate).await.unwrap();
    println!("v {:#?}", v);
}
