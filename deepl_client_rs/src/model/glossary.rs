use serde::{Serialize, Deserialize};
use super::{GlossaryId, GlossarySourceLanguage, GlossaryTargetLanguage};
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct Glossary {
    ///The creation time of the glossary in the ISO 8601-1:2019 format (e.g.: `2021-08-03T14:16:18.329Z`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The number of entries in the glossary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_count: Option<i64>,
    ///A unique ID assigned to a glossary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary_id: Option<GlossaryId>,
    ///Name associated with the glossary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**Indicates if the newly created glossary can already be used in `translate` requests.
If the created glossary is not yet ready, you have to wait and check the `ready` status
of the glossary before using it in a `translate` request.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    ///The language in which the source texts in the glossary are specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<GlossarySourceLanguage>,
    ///The language in which the target texts in the glossary are specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_lang: Option<GlossaryTargetLanguage>,
}
impl std::fmt::Display for Glossary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}