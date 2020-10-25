use crate::api::error::APIError;
use crate::api::{APIClient, FormUrlEncoded};
use crate::loader_task::LoadingFunction;
use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::Callback;

impl APIClient {
    pub fn password_reset_request(
        &self,
        email: String,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("email", email);
        self.post(
            "password_reset/request",
            vec![],
            FormUrlEncoded(body),
            Some(loader),
            callback,
        )
    }
    pub fn password_reset_submit(
        &self,
        email: String,
        token: String,
        new_password: String,
        loader: LoadingFunction,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("email", email);
        body.insert("token", token);
        body.insert("new_password", new_password);
        self.post(
            "password_reset/submit",
            vec![],
            FormUrlEncoded(body),
            Some(loader),
            callback,
        )
    }
}
