use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::get_usage`].

On request success, this will return a [`GetUsageResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct GetUsageRequest {}
impl GetUsageRequest {}
impl FluentRequest<'_, GetUsageRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetUsageRequest> {
    type Output = httpclient::InMemoryResult<GetUsageResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/usage";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}