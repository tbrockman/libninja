#![allow(unused_imports)]
use deepl_client_rs::DeeplClient;
use deepl_client_rs::model::*;
#[tokio::main]
async fn main() {
    let client = DeeplClient::from_env();
    let document_id = "your document id";
    let document_key = "your document key";
    let response = client.get_document_status(document_id, document_key).await.unwrap();
    println!("{:#?}", response);
}