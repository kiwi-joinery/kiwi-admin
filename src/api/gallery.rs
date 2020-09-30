use crate::api::error::APIError;
use crate::api::APIClient;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;
use yew::services::fetch::FetchTask;
use yew::Callback;

pub type GalleryListResponse = HashMap<String, Vec<GalleryItemResponse>>;

#[derive(Deserialize)]
pub struct GalleryItemResponse {
    pub id: i32,
    pub description: String,
    pub category: String,
    pub files: Vec<GalleryFileResponse>,
}

#[derive(Deserialize)]
pub struct GalleryFileResponse {
    url: Url,
    height: i32,
    width: i32,
    bytes: i64,
}

impl APIClient {
    pub fn gallery_list(
        &self,
        callback: Callback<Result<GalleryListResponse, APIError>>,
    ) -> FetchTask {
        self.get("gallery/list", vec![], callback)
    }

    // pub fn gallery_create(
    //     &self,
    //     description: String,
    //     category: String,
    //     callback: Callback<Result<(), APIError>>,
    // ) -> FetchTask {
    //     self.request("/users", vec![], Method::POST, body, callback)
    // }
    //
    // pub fn users_update(
    //     &self,
    //     id: i32,
    //     name: String,
    //     email: String,
    //     callback: Callback<Result<UserResponseItem, APIError>>,
    // ) -> FetchTask {
    //     let mut body = HashMap::new();
    //     body.insert("name", name);
    //     body.insert("email", email);
    //     self.put(&format!("/users/{}", id), vec![], body, callback)
    // }
    //
    // pub fn users_delete(
    //     &self,
    //     id: i32,
    //     callback: Callback<Result<UserResponseItem, APIError>>,
    // ) -> FetchTask {
    //     self.delete(&format!("/users/{}", id), vec![], callback)
    // }
}
