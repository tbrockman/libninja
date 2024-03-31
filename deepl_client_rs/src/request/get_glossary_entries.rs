use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::get_glossary_entries`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct GetGlossaryEntriesRequest {
    pub accept: Option<String>,
    pub glossary_id: String,
}
impl GetGlossaryEntriesRequest {}
impl FluentRequest<'_, GetGlossaryEntriesRequest> {
    ///Set the value of the accept field.
    pub fn accept(mut self, accept: &str) -> Self {
        self.params.accept = Some(accept.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetGlossaryEntriesRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/glossaries/{glossary_id}/entries", glossary_id = self.params
                .glossary_id
            );
            let mut r = self.client.client.get(url);
            if let Some(ref unwrapped) = self.params.accept {
                r = r.header("Accept", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}