use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::download_document`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct DownloadDocumentRequest {
    pub document_id: String,
    pub document_key: String,
}
impl DownloadDocumentRequest {}
impl FluentRequest<'_, DownloadDocumentRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DownloadDocumentRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/document/{document_id}/result", document_id = self.params.document_id
            );
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "document_key" : self.params.document_key }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}