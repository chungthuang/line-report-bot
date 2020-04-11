use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys;

pub struct Request<T: ?Sized>
where
    T: Serialize,
{
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: T,
}

pub async fn post<T>(req: Request<T>) -> Result<web_sys::Response, PostError>
where
    T: Serialize,
{
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    // Equivalent to JSON.stringify in JS
    let body = JsValue::from_str(&serde_json::to_string(&req.body)?);
    opts.body(Some(&body));

    let request = web_sys::Request::new_with_str_and_init(&req.url, &opts)?;
    for (k, v) in req.headers.iter() {
        request.headers().set(k, v)?;
    }

    let window = worker_global_scope().ok_or(PostError::NoWindow)?;

    // `resp_value` is a JS `Response` object.
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: web_sys::Response = resp_value.dyn_into()?;
    Ok(resp)
}

// Returns global execution context of a service worker
fn worker_global_scope() -> Option<web_sys::ServiceWorkerGlobalScope> {
    js_sys::global()
        .dyn_into::<web_sys::ServiceWorkerGlobalScope>()
        .ok()
}

#[derive(Debug)]
pub enum PostError {
    Jv(JsValue),
    SerdeJson(serde_json::error::Error),
    NoWindow,
}

impl From<PostError> for JsValue {
    fn from(pe: PostError) -> Self {
        match pe {
            PostError::Jv(jv) => jv,
            PostError::SerdeJson(e) => JsValue::from_str(&e.to_string()),
            PostError::NoWindow => {
                JsValue::from_str("The runtime doesn't expose a Window interface")
            }
        }
    }
}

impl From<serde_json::error::Error> for PostError {
    fn from(e: serde_json::error::Error) -> Self {
        PostError::SerdeJson(e)
    }
}

impl From<JsValue> for PostError {
    fn from(e: JsValue) -> Self {
        PostError::Jv(e)
    }
}
