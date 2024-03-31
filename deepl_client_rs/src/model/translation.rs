use serde::{Serialize, Deserialize};
use super::SourceLanguage;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct Translation {
    ///The language detected in the source text. It reflects the value of the `source_lang` parameter, when specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detected_source_language: Option<SourceLanguage>,
    ///The translated text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}