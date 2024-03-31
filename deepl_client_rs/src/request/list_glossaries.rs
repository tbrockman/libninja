use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::list_glossaries`].

On request success, this will return a [`ListGlossariesResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct ListGlossariesRequest {}
impl ListGlossariesRequest {}
impl FluentRequest<'_, ListGlossariesRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ListGlossariesRequest> {
    type Output = httpclient::InMemoryResult<ListGlossariesResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/glossaries";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}