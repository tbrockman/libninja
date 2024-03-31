use serde::{Serialize, Deserialize};
use super::SupportedLanguag;
use oasgen::OaSchema;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct ListGlossaryLanguagesResponse {
    ///The list of supported languages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<SupportedLanguag>>,
}
impl std::fmt::Display for ListGlossaryLanguagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}