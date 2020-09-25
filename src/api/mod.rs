use crate::api::error::resolve;
use headers::authorization::Basic;
use headers::{Authorization, HeaderMapExt};
use http::{Request, Response};
use serde::Deserialize;
use url::Url;
use yew::format::Text;
use yew::services::fetch::{FetchService, FetchTask};
use yew::Callback;

mod error;

pub struct APIClient {
    base_url: Url,
    auth_header: Option<Authorization<Basic>>,
}

impl APIClient {
    pub fn new(base_url: Url) -> Self {
        APIClient {
            base_url,
            auth_header: None,
        }
    }

    pub fn add_auth_header(&mut self, a: Authorization<Basic>) {
        self.auth_header = Some(a)
    }

    pub fn remove_auth_header(&mut self) {
        self.auth_header = None
    }

    pub fn request<B, T>(
        &self,
        path: &str,
        method: &str,
        body: B,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: Into<Text>,
        for<'de> T: Deserialize<'de> + 'static,
    {
        let url = self.base_url.join(path).unwrap();
        let mut builder = Request::builder().method(method).uri(url.as_str());
        self.auth_header.as_ref().map(|a| {
            builder.headers_mut().unwrap().typed_insert(a.clone());
        });
        let handler = move |response: Response<Text>| callback.emit(resolve(response));
        FetchService::fetch(builder.body(body).unwrap(), handler.into()).unwrap()
    }
}
