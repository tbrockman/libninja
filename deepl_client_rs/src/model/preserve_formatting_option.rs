use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**Sets whether the translation engine should respect the original formatting, even if it would usually correct some aspects.

The formatting aspects affected by this setting include:
 * Punctuation at the beginning and end of the sentence
 * Upper/lower case at the beginning of the sentence*/
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct PreserveFormattingOption(pub bool);