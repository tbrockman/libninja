#![allow(unused_imports)]
use deepl_client_rs::DeeplClient;
use deepl_client_rs::model::*;
#[tokio::main]
async fn main() {
    let client = DeeplClient::from_env();
    let response = client.list_glossary_languages().await.unwrap();
    println!("{:#?}", response);
}