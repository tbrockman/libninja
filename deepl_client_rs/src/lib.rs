//! [`DeeplClient`](struct.DeeplClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
pub use httpclient::{Error, Result, InMemoryResponseExt};
use std::sync::{Arc, OnceLock};
use std::borrow::Cow;
use crate::model::*;
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
mod serde;
static SHARED_HTTPCLIENT: OnceLock<httpclient::Client> = OnceLock::new();
pub fn default_http_client() -> httpclient::Client {
    httpclient::Client::new()
        .base_url(
            std::env::var("DEEPL_BASE_URL")
                .expect("Missing environment variable DEEPL_BASE_URL")
                .as_str(),
        )
}
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: httpclient::Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}
fn shared_http_client() -> Cow<'static, httpclient::Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a DeeplClient,
    pub params: T,
}
pub struct DeeplClient {
    client: Cow<'static, httpclient::Client>,
    authentication: DeeplAuth,
}
impl DeeplClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
            authentication: DeeplAuth::from_env(),
        }
    }
    pub fn with_auth(authentication: DeeplAuth) -> Self {
        Self {
            client: shared_http_client(),
            authentication,
        }
    }
    pub fn new_with(client: httpclient::Client, authentication: DeeplAuth) -> Self {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    }
}
impl DeeplClient {
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            DeeplAuth::Authorization { authorization } => {
                r = r.header("Authorization", authorization);
            }
        }
        r
    }
    /**Request Translation

The translate function.


The total request body size must not exceed 128 KiB (128 · 1024 bytes). Please split up your text into multiple
calls if it exceeds this limit.*/
    pub fn translate_text(
        &self,
        target_lang: TargetLanguageText,
        text: &[&str],
    ) -> FluentRequest<'_, request::TranslateTextRequest> {
        FluentRequest {
            client: self,
            params: request::TranslateTextRequest {
                context: None,
                formality: None,
                glossary_id: None,
                ignore_tags: None,
                non_splitting_tags: None,
                outline_detection: None,
                preserve_formatting: None,
                source_lang: None,
                split_sentences: None,
                splitting_tags: None,
                tag_handling: None,
                target_lang,
                text: text.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    /**Upload and Translate a Document

This call uploads a document and queues it for translation.
The call returns once the upload is complete, returning a document ID and key which can be used to
[query the translation status](https://www.deepl.com/docs-api/documents/get-document-status)
and to [download the translated document](https://www.deepl.com/docs-api/documents/download-document) once translation is complete.



Because the request includes a file upload, it must be an HTTP POST request with content type `multipart/form-data`.


Please be aware that the uploaded document is automatically removed from the server once the translated document has been downloaded.
You have to upload the document again in order to restart the translation.


The maximum upload limit for documents is [available here](https://support.deepl.com/hc/articles/360020582359-Document-formats)
and may vary based on API plan and document type.


You may specify the glossary to use for the document translation using the `glossary_id` parameter.
**Important:** This requires the `source_lang` parameter to be set and the language pair of the glossary has to match the language pair of the request.*/
    pub fn translate_document(
        &self,
    ) -> FluentRequest<'_, request::TranslateDocumentRequest> {
        FluentRequest {
            client: self,
            params: request::TranslateDocumentRequest {
            },
        }
    }
    /**Check Document Status

Retrieve the current status of a document translation process.
If the translation is still in progress, the estimated time remaining is also included in the response.*/
    pub fn get_document_status(
        &self,
        document_id: &str,
        document_key: &str,
    ) -> FluentRequest<'_, request::GetDocumentStatusRequest> {
        FluentRequest {
            client: self,
            params: request::GetDocumentStatusRequest {
                document_id: document_id.to_owned(),
                document_key: document_key.to_owned(),
            },
        }
    }
    /**Download Translated Document

Once the status of the document translation process is `done`, the result can be downloaded.


For privacy reasons the translated document is automatically removed from the server once it was downloaded and cannot be downloaded again.*/
    pub fn download_document(
        &self,
        document_id: &str,
        document_key: &str,
    ) -> FluentRequest<'_, request::DownloadDocumentRequest> {
        FluentRequest {
            client: self,
            params: request::DownloadDocumentRequest {
                document_id: document_id.to_owned(),
                document_key: document_key.to_owned(),
            },
        }
    }
    /**List Language Pairs Supported by Glossaries

Retrieve the list of language pairs supported by the glossary feature.*/
    pub fn list_glossary_languages(
        &self,
    ) -> FluentRequest<'_, request::ListGlossaryLanguagesRequest> {
        FluentRequest {
            client: self,
            params: request::ListGlossaryLanguagesRequest {
            },
        }
    }
    /**List all Glossaries

List all glossaries and their meta-information, but not the glossary entries.*/
    pub fn list_glossaries(&self) -> FluentRequest<'_, request::ListGlossariesRequest> {
        FluentRequest {
            client: self,
            params: request::ListGlossariesRequest {},
        }
    }
    ///Create a Glossary
    pub fn create_glossary(
        &self,
        args: request::CreateGlossaryRequired,
    ) -> FluentRequest<'_, request::CreateGlossaryRequest> {
        FluentRequest {
            client: self,
            params: request::CreateGlossaryRequest {
                entries: args.entries.to_owned(),
                entries_format: args.entries_format.to_owned(),
                name: args.name.to_owned(),
                source_lang: args.source_lang,
                target_lang: args.target_lang,
            },
        }
    }
    /**Retrieve Glossary Details

Retrieve meta information for a single glossary, omitting the glossary entries.*/
    pub fn get_glossary(
        &self,
        glossary_id: &str,
    ) -> FluentRequest<'_, request::GetGlossaryRequest> {
        FluentRequest {
            client: self,
            params: request::GetGlossaryRequest {
                glossary_id: glossary_id.to_owned(),
            },
        }
    }
    /**Delete a Glossary

Deletes the specified glossary.*/
    pub fn delete_glossary(
        &self,
        glossary_id: &str,
    ) -> FluentRequest<'_, request::DeleteGlossaryRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteGlossaryRequest {
                glossary_id: glossary_id.to_owned(),
            },
        }
    }
    /**Retrieve Glossary Entries

List the entries of a single glossary in the format specified by the `Accept` header.*/
    pub fn get_glossary_entries(
        &self,
        glossary_id: &str,
    ) -> FluentRequest<'_, request::GetGlossaryEntriesRequest> {
        FluentRequest {
            client: self,
            params: request::GetGlossaryEntriesRequest {
                accept: None,
                glossary_id: glossary_id.to_owned(),
            },
        }
    }
    /**Check Usage and Limits

Retrieve usage information within the current billing period together with the corresponding account limits. Usage is returned for:
- translated characters
- translated documents
- translated documents, team totals (for team accounts only)

Character usage includes both text and document translations, and is measured by the source text length in Unicode code points,
so for example "A", "Δ", "あ", and "深" are each counted as a single character.

Document usage only includes document translations, and is measured in individual documents.

Depending on the user account type, some usage types will be omitted.
Character usage is only included for developer accounts.
Document usage is only included for non-developer accounts, and team-combined document usage is only included for non-developer team accounts.*/
    pub fn get_usage(&self) -> FluentRequest<'_, request::GetUsageRequest> {
        FluentRequest {
            client: self,
            params: request::GetUsageRequest {},
        }
    }
    /**Retrieve Supported Languages

Retrieve the list of languages that are currently supported for translation, either as source or target language, respectively.

As of January 2024, Arabic (AR) is supported as a source and target language for text translation,
but it is not yet supported for document translation. Therefore, Arabic has not yet been added to
the `/languages` endpoint. We will add Arabic to the `/languages` endpoint after document translation
support is added.*/
    pub fn get_languages(&self) -> FluentRequest<'_, request::GetLanguagesRequest> {
        FluentRequest {
            client: self,
            params: request::GetLanguagesRequest {
                type_: None,
            },
        }
    }
}
pub enum DeeplAuth {
    Authorization { authorization: String },
}
impl DeeplAuth {
    pub fn from_env() -> Self {
        Self::Authorization {
            authorization: std::env::var("DEEPL_AUTHORIZATION")
                .expect("Environment variable DEEPL_AUTHORIZATION is not set."),
        }
    }
}