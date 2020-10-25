use crate::api::error::APIError;
use crate::api::users::UserResponseItem;
use crate::api::{APIClient, FormUrlEncoded};
use crate::loader_task::LoadingFunction;
use serde::Deserialize;
use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::Callback;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponseItem,
}

impl APIClient {
    pub fn session_login(
        &self,
        email: String,
        password: String,
        loader: LoadingFunction,
        callback: Callback<Result<LoginResponse, APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("password", password);
        self.post(
            "sessions/login",
            vec![],
            FormUrlEncoded(body),
            Some(loader),
            callback,
        )
    }

    pub fn session_logout(
        &self,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        self.delete("sessions/logout", vec![], Some(loader), callback)
    }
}
