use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, oasgen::OaSchema)]
pub enum GlossaryTargetLanguage {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "zh")]
    Zh,
}
