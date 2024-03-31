use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**Sets whether the translated text should lean towards formal or informal language.
This feature currently only works for target languages
`DE` (German),
`FR` (French),
`IT` (Italian),
`ES` (Spanish),
`NL` (Dutch),
`PL` (Polish),
`PT-BR` and `PT-PT` (Portuguese),
`JA` (Japanese),
and `RU` (Russian).
Learn more about the plain/polite feature for Japanese [here](https://support.deepl.com/hc/en-us/articles/6306700061852-About-the-plain-polite-feature-in-Japanese).
Setting this parameter with a target language that does not support formality will fail,
unless one of the `prefer_...` options are used.
Possible options are:
  * `default` (default)
  * `more` - for a more formal language
  * `less` - for a more informal language
  * `prefer_more` - for a more formal language if available, otherwise fallback to default formality
  * `prefer_less` - for a more informal language if available, otherwise fallback to default formality*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub enum Formality {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "more")]
    More,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "prefer_more")]
    PreferMore,
    #[serde(rename = "prefer_less")]
    PreferLess,
}