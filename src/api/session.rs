use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::APIClient;
use serde::Deserialize;
use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::Callback;

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponseItem,
}

impl APIClient {
    pub fn session_login(
        &self,
        email: String,
        password: String,
        callback: Callback<Result<LoginResponse, APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("password", password);
        self.post("sessions/login", vec![], body, callback)
    }

    pub fn session_logout(&self, callback: Callback<Result<(), APIError>>) -> FetchTask {
        self.delete("/sessions/logout", vec![], callback)
    }
}
