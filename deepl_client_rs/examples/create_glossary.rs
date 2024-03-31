#![allow(unused_imports)]
use deepl_client_rs::DeeplClient;
use deepl_client_rs::model::*;
use deepl_client_rs::request::CreateGlossaryRequired;
#[tokio::main]
async fn main() {
    let client = DeeplClient::from_env();
    let args = CreateGlossaryRequired {
        entries: "your entries",
        entries_format: "your entries format",
        name: "your name",
        source_lang: GlossarySourceLanguage::De,
        target_lang: GlossaryTargetLanguage::De,
    };
    let response = client.create_glossary(args).await.unwrap();
    println!("{:#?}", response);
}