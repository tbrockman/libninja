use oasgen::OaSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct Context(pub String);
