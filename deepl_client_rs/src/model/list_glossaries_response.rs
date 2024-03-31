use serde::{Serialize, Deserialize};
use super::Glossary;
use oasgen::OaSchema;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct ListGlossariesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossaries: Option<Vec<Glossary>>,
}
impl std::fmt::Display for ListGlossariesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}