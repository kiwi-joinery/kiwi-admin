use crate::api::error::resolve;
use headers::authorization::Basic;
use headers::{Authorization, HeaderMapExt};
use http::{Method, Request, Response};
use serde::{Deserialize, Serialize};
use url::Url;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask};
use yew::Callback;

pub mod error;
pub mod gallery;
pub mod session;
pub mod users;

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
        query: Vec<(String, String)>,
        method: Method,
        body: B,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: Into<Text>,
        for<'de> T: Deserialize<'de> + 'static,
    {
        let mut url = self.base_url.join(path).unwrap();
        url.query_pairs_mut().extend_pairs(query);
        let mut builder = Request::builder().method(method).uri(url.as_str());
        self.auth_header.as_ref().map(|a| {
            builder.headers_mut().unwrap().typed_insert(a.clone());
        });
        let handler = move |response: Response<Text>| callback.emit(resolve(response));
        FetchService::fetch(builder.body(body).unwrap(), handler.into()).unwrap()
    }

    #[inline]
    pub fn get<T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::GET, Nothing, callback)
    }

    #[inline]
    pub fn post<B, T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        body: B,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: Serialize,
        for<'de> T: Deserialize<'de> + 'static,
    {
        let body: Text = Json(&body).into();
        self.request(path, query, Method::POST, body, callback)
    }

    #[inline]
    pub fn put<B, T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        body: B,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: Serialize,
        for<'de> T: Deserialize<'de> + 'static,
    {
        let body: Text = Json(&body).into();
        self.request(path, query, Method::PUT, body, callback)
    }

    #[inline]
    pub fn delete<T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::DELETE, Nothing, callback)
    }
}
