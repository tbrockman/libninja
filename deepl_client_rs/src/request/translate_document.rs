use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::translate_document`].

On request success, this will return a [`TranslateDocumentResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct TranslateDocumentRequest {}
impl TranslateDocumentRequest {}
impl FluentRequest<'_, TranslateDocumentRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TranslateDocumentRequest> {
    type Output = httpclient::InMemoryResult<TranslateDocumentResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/document";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}