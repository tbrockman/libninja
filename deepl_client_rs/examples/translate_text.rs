#![allow(unused_imports)]
use deepl_client_rs::DeeplClient;
use deepl_client_rs::model::*;
#[tokio::main]
async fn main() {
    let client = DeeplClient::from_env();
    let target_lang = TargetLanguageText::Ar;
    let text = &["your text"];
    let response = client
        .translate_text(target_lang, text)
        .context(Context("your context".to_owned()))
        .formality(Formality::Default)
        .glossary_id(serde_json::json!({}))
        .ignore_tags(TagList(vec!["your tag list".to_owned()]))
        .non_splitting_tags(TagList(vec!["your tag list".to_owned()]))
        .outline_detection(OutlineDetectionOption(true))
        .preserve_formatting(PreserveFormattingOption(true))
        .source_lang(SourceLanguageText::Ar)
        .split_sentences(SplitSentencesOption::SplitSentencesOption0)
        .splitting_tags(TagList(vec!["your tag list".to_owned()]))
        .tag_handling(TagHandlingOption::Xml)
        .await
        .unwrap();
    println!("{:#?}", response);
}