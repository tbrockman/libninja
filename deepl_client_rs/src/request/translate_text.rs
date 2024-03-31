use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::translate_text`].

On request success, this will return a [`TranslateTextResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct TranslateTextRequest {
    pub context: Option<Context>,
    pub formality: Option<Formality>,
    pub glossary_id: Option<serde_json::Value>,
    pub ignore_tags: Option<IgnoreTagList>,
    pub non_splitting_tags: Option<NonSplittingTagList>,
    pub outline_detection: Option<OutlineDetectionOption>,
    pub preserve_formatting: Option<PreserveFormattingOption>,
    pub source_lang: Option<SourceLanguageText>,
    pub split_sentences: Option<SplitSentencesOption>,
    pub splitting_tags: Option<SplittingTagList>,
    pub tag_handling: Option<TagHandlingOption>,
    pub target_lang: TargetLanguageText,
    pub text: Vec<String>,
}
impl TranslateTextRequest {}
impl FluentRequest<'_, TranslateTextRequest> {
    ///Set the value of the context field.
    pub fn context(mut self, context: Context) -> Self {
        self.params.context = Some(context);
        self
    }
    ///Set the value of the formality field.
    pub fn formality(mut self, formality: Formality) -> Self {
        self.params.formality = Some(formality);
        self
    }
    ///Set the value of the glossary_id field.
    pub fn glossary_id(mut self, glossary_id: serde_json::Value) -> Self {
        self.params.glossary_id = Some(glossary_id);
        self
    }
    ///Set the value of the ignore_tags field.
    pub fn ignore_tags(mut self, ignore_tags: IgnoreTagList) -> Self {
        self.params.ignore_tags = Some(ignore_tags);
        self
    }
    ///Set the value of the non_splitting_tags field.
    pub fn non_splitting_tags(
        mut self,
        non_splitting_tags: NonSplittingTagList,
    ) -> Self {
        self.params.non_splitting_tags = Some(non_splitting_tags);
        self
    }
    ///Set the value of the outline_detection field.
    pub fn outline_detection(
        mut self,
        outline_detection: OutlineDetectionOption,
    ) -> Self {
        self.params.outline_detection = Some(outline_detection);
        self
    }
    ///Set the value of the preserve_formatting field.
    pub fn preserve_formatting(
        mut self,
        preserve_formatting: PreserveFormattingOption,
    ) -> Self {
        self.params.preserve_formatting = Some(preserve_formatting);
        self
    }
    ///Set the value of the source_lang field.
    pub fn source_lang(mut self, source_lang: SourceLanguageText) -> Self {
        self.params.source_lang = Some(source_lang);
        self
    }
    ///Set the value of the split_sentences field.
    pub fn split_sentences(mut self, split_sentences: SplitSentencesOption) -> Self {
        self.params.split_sentences = Some(split_sentences);
        self
    }
    ///Set the value of the splitting_tags field.
    pub fn splitting_tags(mut self, splitting_tags: SplittingTagList) -> Self {
        self.params.splitting_tags = Some(splitting_tags);
        self
    }
    ///Set the value of the tag_handling field.
    pub fn tag_handling(mut self, tag_handling: TagHandlingOption) -> Self {
        self.params.tag_handling = Some(tag_handling);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TranslateTextRequest> {
    type Output = httpclient::InMemoryResult<TranslateTextResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/translate";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.context {
                r = r.json(json!({ "context" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.formality {
                r = r.json(json!({ "formality" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.glossary_id {
                r = r.json(json!({ "glossary_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.ignore_tags {
                r = r.json(json!({ "ignore_tags" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.non_splitting_tags {
                r = r.json(json!({ "non_splitting_tags" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.outline_detection {
                r = r.json(json!({ "outline_detection" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.preserve_formatting {
                r = r.json(json!({ "preserve_formatting" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.source_lang {
                r = r.json(json!({ "source_lang" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.split_sentences {
                r = r.json(json!({ "split_sentences" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.splitting_tags {
                r = r.json(json!({ "splitting_tags" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.tag_handling {
                r = r.json(json!({ "tag_handling" : unwrapped }));
            }
            r = r.json(json!({ "target_lang" : self.params.target_lang }));
            r = r.json(json!({ "text" : self.params.text }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}