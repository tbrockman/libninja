use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, oasgen::OaSchema)]
pub enum Formality {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "more")]
    More,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "prefer_more")]
    PreferMore,
    #[serde(rename = "prefer_less")]
    PreferLess,
}
