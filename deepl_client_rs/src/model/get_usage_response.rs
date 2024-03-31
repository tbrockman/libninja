use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct GetUsageResponse {
    ///Characters translated so far in the current billing period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_count: Option<i64>,
    ///Current maximum number of characters that can be translated per billing period. If cost control is set, the cost control limit will be returned in this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_limit: Option<i64>,
}
impl std::fmt::Display for GetUsageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}