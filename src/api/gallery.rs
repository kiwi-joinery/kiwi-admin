use crate::api::error::APIError;
use crate::api::multipart::{Multipart, MultipartFile};
use crate::api::{APIClient, FormUrlEncoded};
use crate::loader_task::LoadingFunction;
use enum_iterator::IntoEnumIterator;
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Formatter;
use url::Url;
use yew::services::fetch::FetchTask;
use yew::services::reader::FileData;
use yew::Callback;

#[derive(Debug, Deserialize, Serialize, IntoEnumIterator, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum Category {
    Staircases,
    Windows,
    Doors,
    Other,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Staircases => f.write_str("Staircases"),
            Category::Windows => f.write_str("Windows"),
            Category::Doors => f.write_str("Doors"),
            Category::Other => f.write_str("Other"),
        }
    }
}

impl Category {
    pub fn serialize(&self) -> String {
        serde_plain::to_string(&self).unwrap()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GalleryItemResponse {
    pub id: u32,
    pub description: String,
    pub category: Category,
    pub files: Vec<GalleryFileResponse>,
}

impl GalleryItemResponse {
    pub fn image_with_width_geq(&self, width: u32) -> Option<&GalleryFileResponse> {
        self.files
            .iter()
            .filter(|f| f.width >= width)
            .min_by_key(|f| f.width)
    }
    pub fn image_with_width_leq(&self, width: u32) -> Option<&GalleryFileResponse> {
        self.files
            .iter()
            .filter(|f| f.width <= width)
            .max_by_key(|f| f.width)
    }
    pub fn best_matching_width(&self, width: u32) -> Option<&GalleryFileResponse> {
        self.image_with_width_geq(width)
            .or(self.image_with_width_leq(width))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GalleryFileResponse {
    pub url: Url,
    pub height: u32,
    pub width: u32,
    pub bytes: u32,
}

pub type GalleryListResponse = HashMap<Category, Vec<GalleryItemResponse>>;

impl APIClient {
    pub fn gallery_list(
        &self,
        loader: LoadingFunction,
        callback: Callback<Result<GalleryListResponse, APIError>>,
    ) -> FetchTask {
        self.get("gallery/list", vec![], Some(loader), callback)
    }

    pub fn gallery_create(
        &self,
        image: &FileData,
        description: String,
        category: &Category,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        let mut form = Multipart::new();
        form.add_text("description", description);
        form.add_text("category", category.serialize());
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
    pub fn gallery_update(
        &self,
        id: u32,
        description: String,
        category: Category,
        move_after_id: Option<u32>,
        move_to_front: bool,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("description", description);
        body.insert("category", category.serialize());
        match move_after_id {
            None => {}
            Some(id) => {
                body.insert("move_after_id", id.to_string());
            }
        }
        body.insert("move_to_front", move_to_front.to_string());
        self.put(
            &format!("gallery/{}", id),
            vec![],
            FormUrlEncoded(body),
            Some(loader),
            callback,
        )
    }

    pub fn gallery_get(
        &self,
        id: u32,
        loader: LoadingFunction,
        callback: Callback<Result<GalleryItemResponse, APIError>>,
    ) -> FetchTask {
        self.get(&format!("gallery/{}", id), vec![], Some(loader), callback)
    }

    pub fn gallery_delete(
        &self,
        id: u32,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        self.delete(&format!("gallery/{}", id), vec![], Some(loader), callback)
    }
}
