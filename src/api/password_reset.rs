use crate::api::error::APIError;
use crate::api::{APIClient, FormUrlEncoded, ProgressCallback};
use std::collections::HashMap;
use yew::services::fetch::FetchTask;
use yew::Callback;

impl APIClient {
    pub fn password_reset_request(
        &self,
        email: String,
        progress: ProgressCallback,
        callback: Callback<Result<(), APIError>>,
    ) -> FetchTask {
        let mut body = HashMap::new();
        body.insert("email", email);
        self.post(
            "password_reset/request",
            vec![],
            FormUrlEncoded(body),
            Some(progress),
            callback,
        )
    }
    pub fn password_reset_submit(
        &self,
        email: String,
        token: String,
        new_password: String,
        progress: ProgressCallback,
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
            Some(progress),
            callback,
        )
    }
}
