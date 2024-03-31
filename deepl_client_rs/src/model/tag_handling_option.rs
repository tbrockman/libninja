use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**Sets which kind of tags should be handled. Options currently available:
 * `xml`: Enable XML tag handling; see [XML Handling](https://www.deepl.com/docs-api/xml).
 * `html`: Enable HTML tag handling; see [HTML Handling](https://www.deepl.com/docs-api/html).*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub enum TagHandlingOption {
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "html")]
    Html,
}