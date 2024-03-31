#![allow(unused_imports)]
use deepl_client_rs::DeeplClient;
use deepl_client_rs::model::*;
#[tokio::main]
async fn main() {
    let client = DeeplClient::from_env();
    let glossary_id = "your glossary id";
    let response = client.delete_glossary(glossary_id).await.unwrap();
    println!("{:#?}", response);
}