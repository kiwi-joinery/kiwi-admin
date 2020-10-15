use crate::api::error::resolve;
use crate::components::loading::LoadingProps;
use headers::authorization::Basic;
use headers::{Authorization, ContentType, HeaderMapExt};
use http::{Method, Request, Response};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;
use yew::format::{Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask};
use yew::Callback;

pub mod error;
pub mod gallery;
pub mod password_reset;
pub mod session;
pub mod users;

#[derive(PartialEq, Clone)]
pub struct APIClient {
    base_url: Url,
    auth_header: Option<Authorization<Basic>>,
}

pub type ProgressCallback = Callback<LoadingProps>;

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
        progress: Option<Callback<LoadingProps>>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: TextBody,
        for<'de> T: Deserialize<'de> + 'static,
    {
        match &progress {
            None => {}
            Some(p) => p.emit(LoadingProps::enabled(None)),
        }
        let mut url = self.base_url.join(path).unwrap();
        url.query_pairs_mut().extend_pairs(query);
        let mut builder = Request::builder().method(method).uri(url.as_str());
        match B::content_type() {
            Some(c) => builder.headers_mut().unwrap().typed_insert(c),
            None => {}
        }
        match self.auth_header.as_ref() {
            Some(a) => builder.headers_mut().unwrap().typed_insert(a.clone()),
            None => {}
        }
        let handler = move |response: Response<Text>| {
            match progress.as_ref() {
                None => {}
                Some(p) => p.emit(LoadingProps::disabled()),
            }
            callback.emit(resolve(response));
        };
        FetchService::fetch(builder.body(body).unwrap(), handler.into()).unwrap()
    }

    #[inline]
    pub fn get<T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        progress: Option<ProgressCallback>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::GET, Empty, progress, callback)
    }

    #[inline]
    pub fn post<B, T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        body: B,
        progress: Option<ProgressCallback>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: TextBody,
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::POST, body, progress, callback)
    }

    #[inline]
    pub fn put<B, T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        body: B,
        progress: Option<ProgressCallback>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        B: TextBody,
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::PUT, body, progress, callback)
    }

    #[inline]
    pub fn delete<T>(
        &self,
        path: &str,
        query: Vec<(String, String)>,
        progress: Option<ProgressCallback>,
        callback: Callback<Result<T, error::APIError>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
    {
        self.request(path, query, Method::DELETE, Empty, progress, callback)
    }
}

pub trait TextBody: Into<Text> {
    fn content_type() -> Option<ContentType>;
}

pub struct Empty;

impl TextBody for Empty {
    fn content_type() -> Option<ContentType> {
        None
    }
}

impl Into<Text> for Empty {
    fn into(self) -> Text {
        Nothing.into()
    }
}

pub struct FormUrlEncoded<T>(T);

impl<T: Serialize> TextBody for FormUrlEncoded<T> {
    fn content_type() -> Option<ContentType> {
        Some(ContentType::form_url_encoded())
    }
}

impl<T: Serialize> Into<Text> for FormUrlEncoded<T> {
    fn into(self) -> Text {
        Ok(serde_urlencoded::to_string(self.0).unwrap())
    }
}
