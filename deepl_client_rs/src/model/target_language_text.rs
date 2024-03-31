use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**The language into which the text should be translated. Options currently available:
 * `AR` - Arabic [1]
 * `BG` - Bulgarian
 * `CS` - Czech
 * `DA` - Danish
 * `DE` - German
 * `EL` - Greek
 * `EN` - English (unspecified variant for backward compatibility; please select `EN-GB` or `EN-US` instead)
 * `EN-GB` - English (British)
 * `EN-US` - English (American)
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
 * `PT` - Portuguese (unspecified variant for backward compatibility; please select `PT-BR` or `PT-PT` instead)
 * `PT-BR` - Portuguese (Brazilian)
 * `PT-PT` - Portuguese (all Portuguese varieties excluding Brazilian Portuguese)
 * `RO` - Romanian
 * `RU` - Russian
 * `SK` - Slovak
 * `SL` - Slovenian
 * `SV` - Swedish
 * `TR` - Turkish
 * `UK` - Ukrainian
 * `ZH` - Chinese (simplified)

 [1] Please note that Arabic has not yet been added to the `/languages` endpoint because
 it does not yet support document translation; only text translation is supported for Arabic
 at this time. When document translation support is added for Arabic, we will a) remove this
 note and b) add Arabic to the `/languages` endpoint.*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub enum TargetLanguageText {
    #[serde(rename = "AR")]
    Ar,
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
    #[serde(rename = "EN-GB")]
    EnGb,
    #[serde(rename = "EN-US")]
    EnUs,
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
    #[serde(rename = "PT-BR")]
    PtBr,
    #[serde(rename = "PT-PT")]
    PtPt,
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