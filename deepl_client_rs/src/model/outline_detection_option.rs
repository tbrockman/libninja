use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**The automatic detection of the XML structure won't yield best results in all XML files. You can disable this automatic mechanism altogether by setting the `outline_detection` parameter to `false` and selecting the tags that should be considered structure tags. This will split sentences using the `splitting_tags` parameter.


In the example below, we achieve the same results as the automatic engine by disabling automatic detection with `outline_detection=0` and setting the parameters manually to `tag_handling=xml`, `split_sentences=nonewlines`,  and `splitting_tags=par,title`.
 * Example request:
   ```
   <document>
     <meta>
       <title>A document's title</title>
     </meta>
     <content>
       <par>This is the first sentence. Followed by a second one.</par>
       <par>This is the third sentence.</par>
     </content>
   </document>
   ```
 * Example response:
   ```
   <document>
     <meta>
       <title>Der Titel eines Dokuments</title>
     </meta>
     <content>
       <par>Das ist der erste Satz. Gefolgt von einem zweiten.</par>
       <par>Dies ist der dritte Satz.</par>
     </content>
   </document>
   ```
While this approach is slightly more complicated, it allows for greater control over the structure of the translation output.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct OutlineDetectionOption(pub bool);