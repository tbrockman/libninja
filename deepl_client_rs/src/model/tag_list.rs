use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
///List of XML or HTML tags.
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct TagList(pub Vec<String>);