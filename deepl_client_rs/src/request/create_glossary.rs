use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::create_glossary`].

On request success, this will return a [`Glossary`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct CreateGlossaryRequest {
    pub entries: String,
    pub entries_format: String,
    pub name: String,
    pub source_lang: GlossarySourceLanguage,
    pub target_lang: GlossaryTargetLanguage,
}
impl CreateGlossaryRequest {}
pub struct CreateGlossaryRequired<'a> {
    pub entries: &'a str,
    pub entries_format: &'a str,
    pub name: &'a str,
    pub source_lang: GlossarySourceLanguage,
    pub target_lang: GlossaryTargetLanguage,
}
impl<'a> CreateGlossaryRequired<'a> {}
impl FluentRequest<'_, CreateGlossaryRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreateGlossaryRequest> {
    type Output = httpclient::InMemoryResult<Glossary>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/glossaries";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "entries" : self.params.entries }));
            r = r.json(json!({ "entries_format" : self.params.entries_format }));
            r = r.json(json!({ "name" : self.params.name }));
            r = r.json(json!({ "source_lang" : self.params.source_lang }));
            r = r.json(json!({ "target_lang" : self.params.target_lang }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}