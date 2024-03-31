use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct SupportedLanguag {
    ///The language in which the source texts in the glossary are specified.
    pub source_lang: String,
    ///The language in which the target texts in the glossary are specified.
    pub target_lang: String,
}
impl std::fmt::Display for SupportedLanguag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}