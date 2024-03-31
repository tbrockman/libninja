use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**Sets whether the translation engine should first split the input into sentences. For text translations where
`tag_handling` is not set to `html`, the default value is `1`, meaning the engine splits on punctuation and on newlines.

For text translations where `tag_handling=html`, the default value is `nonewlines`, meaning the engine splits on punctuation only, ignoring newlines.

The use of `nonewlines` as the default value for text translations where `tag_handling=html` is new behavior that was implemented in November 2022,
when HTML handling was moved out of beta.

Possible values are:

 * `0` - no splitting at all, whole input is treated as one sentence
 * `1` (default when `tag_handling` is not set to `html`) - splits on punctuation and on newlines
 * `nonewlines` (default when `tag_handling=html`) - splits on punctuation only, ignoring newlines

For applications that send one sentence per text parameter, we recommend setting `split_sentences` to `0`, in order to prevent the engine from splitting the sentence unintentionally.


Please note that newlines will split sentences when `split_sentences=1`. We recommend cleaning files so they don't contain breaking sentences or setting the parameter `split_sentences` to `nonewlines`.*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub enum SplitSentencesOption {
    #[serde(rename = "0")]
    SplitSentencesOption0,
    #[serde(rename = "1")]
    SplitSentencesOption1,
    #[serde(rename = "nonewlines")]
    Nonewlines,
}