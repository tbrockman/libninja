use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
///A unique ID assigned to a glossary.
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct GlossaryId(pub String);