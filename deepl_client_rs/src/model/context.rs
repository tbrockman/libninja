use serde::{Serialize, Deserialize};
use oasgen::OaSchema;
/**The `context` parameter makes it possible to include additional context that can influence a translation but is not translated itself.
This additional context can potentially improve translation quality when translating short, low-context source texts such
as product names on an e-commerce website, article headlines on a news website, or UI elements.


For example...
  - When translating a product name, you might pass the product description as context.
  - When translating a news article headline, you might pass the first few sentences or a summary of the article as context.


For best results, we recommend sending a few complete sentences of context. There is no size limit for the `context` parameter itself, but
the request body size limit of 128 KiB still applies to all text translation requests.


If you send a request with multiple `text` parameters, the `context` parameter will be applied to each one.


The `context` parameter is an **alpha feature** as defined in section 3.1.5 of our [terms and conditions](https://www.deepl.com/en/pro-license).
This means it could be changed or deprecated by DeepL at any point and without advance notice. While the feature is still labeled as "alpha",
context will not be counted toward billing (i.e. there is no additional cost for sending context). This is subject to change if the "alpha"
label is removed and the feature becomes generally available. If we decide to deprecate the `context` parameter, requests that include it will not
break; the context will simply be ignored.


For these reasons, **the `context` parameter is not intended to be used in production** as long as the alpha label still applies.


We're eager to hear how the `context` parameter is working for you and how we can improve the feature! You can share your feedback
by emailing api-feedback@deepl.com.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default, oasgen::OaSchema)]
pub struct Context(pub String);