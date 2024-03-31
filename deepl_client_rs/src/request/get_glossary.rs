use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::get_glossary`].

On request success, this will return a [`Glossary`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct GetGlossaryRequest {
    pub glossary_id: String,
}
impl GetGlossaryRequest {}
impl FluentRequest<'_, GetGlossaryRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetGlossaryRequest> {
    type Output = httpclient::InMemoryResult<Glossary>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/glossaries/{glossary_id}", glossary_id = self.params.glossary_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}