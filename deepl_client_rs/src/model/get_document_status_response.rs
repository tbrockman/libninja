use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct GetDocumentStatusResponse {
    ///The number of characters billed to your account. The characters will only be billed after a successful download request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billed_characters: Option<i64>,
    ///A unique ID assigned to the uploaded document and the requested translation process. The same ID that was used when requesting the translation status.
    pub document_id: String,
    /**A short description of the error, if available.
Note that the content is subject to change.
This parameter may be included if an error occurred during translation.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /**Estimated number of seconds until the translation is done.
This parameter is only included while `status` is `"translating"`.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds_remaining: Option<i64>,
    /**A short description of the state the document translation process is currently in. Possible values are:
 * `queued` - the translation job is waiting in line to be processed
 * `translating` - the translation is currently ongoing
 * `done` - the translation is done and the translated document is ready for download
 * `error` - an irrecoverable error occurred while translating the document*/
    pub status: String,
}
impl std::fmt::Display for GetDocumentStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}