use crate::api::error::resolve;
use crate::loader_task::{BoxedLoadingTask, LoadingFunction, LoadingTaskConfig};
use headers::authorization::Basic;
use headers::{Authorization, ContentType, HeaderMapExt};
use http::{Method, Request, Response};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;
use yew::format::{Binary, Nothing};
use yew::services::fetch::{FetchService, FetchTask};
use yew::Callback;

pub mod error;
pub mod gallery;
mod multipart;
pub mod password_reset;
pub mod session;
pub mod users;

#[derive(PartialEq, Clone)]
pub struct APIClient {
    base_url: Url,
    auth_header: Option<Authorization<Basic>>,
}

impl APIClient {
    pub fn new(base_url: &str) -> Self {
        APIClient {
            base_url: Url::from_str(base_url).unwrap(),
            auth_header: None,
        }
    }

    pub fn auth_header(&self) -> &Option<Authorization<Basic>> {
        &self.auth_header
    }

    pub fn set_auth_header(&mut self, a: Authorization<Basic>) {
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
        loader: Option<LoadingFunction>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: RequestBody,
        for<'de> T: Deserialize<'de> + 'static,
    {
        let loader_task: Option<BoxedLoadingTask> = loader.map(|x| x(LoadingTaskConfig::default()));
        let mut url = self.base_url.join(path).unwrap();
        url.query_pairs_mut().extend_pairs(query);
        let mut builder = Request::builder().method(method).uri(url.as_str());
        match body.content_type() {
            Some(c) => builder.headers_mut().unwrap().typed_insert(c),
            None => {}
        }
        match self.auth_header.as_ref() {
            Some(a) => builder.headers_mut().unwrap().typed_insert(a.clone()),
            None => {}
        }
        let request: Binary = body.into();
        let handler = move |response: Response<Binary>| {
            drop(loader_task);
            callback.emit(resolve(response));
        };
        FetchService::fetch_binary(builder.body(request).unwrap(), Callback::once(handler)).unwrap()
    }

    #[inline]
    pub fn get<T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        loader: Option<LoadingFunction>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::GET, Empty, loader, callback)
    }

    #[inline]
    pub fn post<B, T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        body: B,
        loader: Option<LoadingFunction>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: RequestBody,
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::POST, body, loader, callback)
    }

    #[inline]
    pub fn put<B, T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        body: B,
        loader: Option<LoadingFunction>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: RequestBody,
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::PUT, body, loader, callback)
    }

    #[inline]
    pub fn delete<T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        loader: Option<LoadingFunction>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::DELETE, Empty, loader, callback)
    }
}

pub trait RequestBody: Into<Binary> {
    fn content_type(&self) -> Option<ContentType>;
}

pub struct Empty;

impl RequestBody for Empty {
    fn content_type(&self) -> Option<ContentType> {
        None
    }
}

impl Into<Binary> for Empty {
    fn into(self) -> Binary {
        Nothing.into()
    }
}

pub struct FormUrlEncoded<T>(T);

impl<T: Serialize> RequestBody for FormUrlEncoded<T> {
    fn content_type(&self) -> Option<ContentType> {
        Some(ContentType::form_url_encoded())
    }
}

impl<T: Serialize> Into<Binary> for FormUrlEncoded<T> {
    fn into(self) -> Binary {
        Ok(serde_urlencoded::to_string(self.0).unwrap()).map(|x| x.into_bytes())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Counted<T> {
    pub total: u32,
    pub results: Vec<T>,
}
