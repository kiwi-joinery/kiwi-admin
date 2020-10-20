use crate::api::error::APIError;
use crate::api::{APIClient, Counted, FormUrlEncoded, ProgressCallback};
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
        limit: u32,
        offset: u32,
        search: Option<String>,
        progress: ProgressCallback,
        callback: Callback<Result<Counted<UserResponseItem>, APIError>>,
    ) -> FetchTask {
        let mut query = Vec::new();
        query.push(("limit".to_string(), limit.to_string()));
        query.push(("offset".to_string(), offset.to_string()));
        match search {
            None => {}
            Some(s) => {
                query.push(("search".to_string(), s));
            }
        }
        self.get("users", query, Some(progress), callback)
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
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        self.delete(&format!("users/{}", id), vec![], Some(progress), callback)
    }
}
