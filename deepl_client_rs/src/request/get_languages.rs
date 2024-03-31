use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::DeeplClient;
/**You should use this struct via [`DeeplClient::get_languages`].

On request success, this will return a [`Vec<serde_json::Value>`].*/
#[derive(Debug, Clone, Serialize, Deserialize, oasgen::OaSchema)]
pub struct GetLanguagesRequest {
    pub type_: Option<String>,
}
impl GetLanguagesRequest {}
impl FluentRequest<'_, GetLanguagesRequest> {
    ///Set the value of the type_ field.
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetLanguagesRequest> {
    type Output = httpclient::InMemoryResult<Vec<serde_json::Value>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/languages";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}