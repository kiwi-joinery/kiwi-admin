use crate::api::error::APIError;
use crate::api::multipart::{Multipart, MultipartFile};
use crate::api::APIClient;
use crate::loader_task::LoadingFunction;
use http::Method;
use serde::Deserialize;
use std::fmt::Formatter;
use url::Url;
use yew::services::fetch::FetchTask;
use yew::services::reader::FileData;
use yew::Callback;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Category {
    Staircases,
    Windows,
    Doors,
    Other,
}

#[derive(Deserialize)]
pub struct GalleryItemResponse {
    pub id: u32,
    pub description: String,
    pub category: Category,
    pub files: Vec<GalleryFileResponse>,
}

#[derive(Deserialize)]
pub struct GalleryFileResponse {
    url: Url,
    height: u32,
    width: u32,
    bytes: u32,
}

impl APIClient {
    pub fn gallery_list(
        &self,
        loader: LoadingFunction,
        callback: Callback<Result<Vec<GalleryItemResponse>, APIError>>,
    ) -> FetchTask {
        self.get("gallery/list", vec![], Some(loader), callback)
    }

    pub fn gallery_create(
        &self,
        image: &FileData,
        description: String,
        category: Category,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        let mut form = Multipart::new();
        form.add_text("description", description);
        form.add_text("category", category.to_string());
        form.add_file(MultipartFile::new(
            "image",
            image.content.clone(),
            Some(image.name.clone()),
        ));
        self.request(
            "gallery",
            vec![],
            Method::POST,
            form,
            Some(loader),
            callback,
        )
    }
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

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Staircases => f.write_str("STAIRCASES"),
            Category::Windows => f.write_str("WINDOWS"),
            Category::Doors => f.write_str("DOORS"),
            Category::Other => f.write_str("OTHER"),
        }
    }
}
