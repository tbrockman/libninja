use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**Language of the text to be translated. Options currently available:
 * `BG` - Bulgarian
 * `CS` - Czech
 * `DA` - Danish
 * `DE` - German
 * `EL` - Greek
 * `EN` - English
 * `ES` - Spanish
 * `ET` - Estonian
 * `FI` - Finnish
 * `FR` - French
 * `HU` - Hungarian
 * `ID` - Indonesian
 * `IT` - Italian
 * `JA` - Japanese
 * `KO` - Korean
 * `LT` - Lithuanian
 * `LV` - Latvian
 * `NB` - Norwegian (Bokmål)
 * `NL` - Dutch
 * `PL` - Polish
 * `PT` - Portuguese (all Portuguese varieties mixed)
 * `RO` - Romanian
 * `RU` - Russian
 * `SK` - Slovak
 * `SL` - Slovenian
 * `SV` - Swedish
 * `TR` - Turkish
 * `UK` - Ukrainian
 * `ZH` - Chinese

If this parameter is omitted, the API will attempt to detect the language of the text and translate it.*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub enum SourceLanguage {
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "CS")]
    Cs,
    #[serde(rename = "DA")]
    Da,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "EL")]
    El,
    #[serde(rename = "EN")]
    En,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JA")]
    Ja,
    #[serde(rename = "KO")]
    Ko,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "NB")]
    Nb,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "UK")]
    Uk,
    #[serde(rename = "ZH")]
    Zh,
}