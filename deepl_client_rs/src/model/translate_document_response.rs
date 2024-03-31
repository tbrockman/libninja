use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct TranslateDocumentResponse {
    ///A unique ID assigned to the uploaded document and the translation process. Must be used when referring to this particular document in subsequent API requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///A unique key that is used to encrypt the uploaded document as well as the resulting translation on the server side. Must be provided with every subsequent API request regarding this particular document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_key: Option<String>,
}
impl std::fmt::Display for TranslateDocumentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}