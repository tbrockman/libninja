use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, oasgen::OaSchema)]
pub enum SourceLanguageText {
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "CS")]
    Cs,
    #[serde(rename = "DA")]
    Da,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "EL")]
    El,
    #[serde(rename = "EN")]
    En,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JA")]
    Ja,
    #[serde(rename = "KO")]
    Ko,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "NB")]
    Nb,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "UK")]
    Uk,
    #[serde(rename = "ZH")]
    Zh,
}
