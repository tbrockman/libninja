use serde::{Serialize, Deserialize};
use super::Translation;
use oasgen::OaSchema;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct TranslateTextResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<Translation>>,
}
impl std::fmt::Display for TranslateTextResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}