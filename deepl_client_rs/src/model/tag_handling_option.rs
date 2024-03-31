use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, oasgen::OaSchema)]
pub enum TagHandlingOption {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "html")]
    Html,
}
