#![allow(unused_imports)]
use deepl_client_rs::DeeplClient;
use deepl_client_rs::model::*;
#[tokio::main]
async fn main() {
    let client = DeeplClient::from_env();
    let response = client.get_usage().await.unwrap();
    println!("{:#?}", response);
}