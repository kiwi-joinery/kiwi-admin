use crate::api::error::APIError;
use crate::api::{APIClient, FormUrlEncoded};
use serde::Deserialize;
use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::Callback;

#[derive(Deserialize, Debug)]
pub struct UserResponseItem {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl APIClient {
    pub fn users_list(
        &self,
        callback: Callback<Result<Vec<UserResponseItem>, APIError>>,
    ) -> FetchTask {
        self.get("users", vec![], callback)
    }

    pub fn users_create(
        &self,
        name: String,
        email: String,
        callback: Callback<Result<Vec<UserResponseItem>, APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("name", name);
        body.insert("email", email);
        self.post("users", vec![], FormUrlEncoded(body), callback)
    }

    pub fn users_get(
        &self,
        id: i32,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        self.get(&format!("users/{}", id), vec![], callback)
    }

    pub fn users_update(
        &self,
        id: i32,
        name: String,
        email: String,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("name", name);
        body.insert("email", email);
        self.put(
            &format!("users/{}", id),
            vec![],
            FormUrlEncoded(body),
            callback,
        )
    }

    pub fn users_delete(
        &self,
        id: i32,
        callback: Callback<Result<UserResponseItem, APIError>>,
    ) -> FetchTask {
        self.delete(&format!("users/{}", id), vec![], callback)
    }
}
