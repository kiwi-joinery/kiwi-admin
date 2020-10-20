use crate::api::error::APIError;
use crate::api::{APIClient, FormUrlEncoded, ProgressCallback};
use serde::Deserialize;
use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::Callback;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserResponseItem {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl APIClient {
    pub fn users_list(
        &self,
        progress: ProgressCallback,
        callback: Callback<Result<Vec<UserResponseItem>, APIError>>,
    ) -> FetchTask {
        self.get("users", vec![], Some(progress), callback)
    }

    pub fn users_create(
        &self,
        name: String,
        email: String,
        progress: ProgressCallback,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("name", name);
        body.insert("email", email);
        self.post(
            "users",
            vec![],
            FormUrlEncoded(body),
            Some(progress),
            callback,
        )
    }

    pub fn users_get(
        &self,
        id: u32,
        progress: Option<ProgressCallback>,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        self.get(&format!("users/{}", id), vec![], progress, callback)
    }

    pub fn users_update(
        &self,
        id: u32,
        name: String,
        email: String,
        progress: ProgressCallback,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("name", name);
        body.insert("email", email);
        self.put(
            &format!("users/{}", id),
            vec![],
            FormUrlEncoded(body),
            Some(progress),
            callback,
        )
    }

    pub fn users_delete(
        &self,
        id: u32,
        progress: ProgressCallback,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        self.delete(&format!("users/{}", id), vec![], Some(progress), callback)
    }
}
