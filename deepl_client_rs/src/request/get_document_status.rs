use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::get_document_status`].

On request success, this will return a [`GetDocumentStatusResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct GetDocumentStatusRequest {
    pub document_id: String,
    pub document_key: String,
}
impl GetDocumentStatusRequest {}
impl FluentRequest<'_, GetDocumentStatusRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetDocumentStatusRequest> {
    type Output = httpclient::InMemoryResult<GetDocumentStatusResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/document/{document_id}", document_id = self.params.document_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "document_key" : self.params.document_key }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}