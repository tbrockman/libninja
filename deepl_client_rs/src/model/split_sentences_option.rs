use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, oasgen::OaSchema)]
pub enum SplitSentencesOption {
    #[serde(rename = "0")]
    SplitSentencesOption0,
    #[serde(rename = "1")]
    SplitSentencesOption1,
    #[serde(rename = "nonewlines")]
    Nonewlines,
}
