use headers::authorization::Basic;
use headers::Authorization;
use serde::{Deserialize, Serialize};
use yew::format::{Json, Text};
use yew::services::storage::Area;
use yew::services::StorageService;

#[derive(Serialize, Deserialize, Clone)]
pub struct PersistedAuth {
    pub user_id: u32,
    token: String,
}

impl From<PersistedAuth> for Authorization<Basic> {
    fn from(x: PersistedAuth) -> Self {
        Authorization::basic(&x.user_id.to_string(), &x.token)
    }
}

const KEY: &str = "kiwi_auth";

fn storage() -> StorageService {
    StorageService::new(Area::Local).expect("storage was disabled by the user")
}

impl PersistedAuth {
    pub fn load() -> Option<PersistedAuth> {
        let res = storage().restore::<Text>(KEY);
        res.ok().and_then(|x| serde_json::from_str(&x).ok())
    }

    pub fn persist(user_id: u32, token: String) -> PersistedAuth {
        let x = Self { user_id, token };
        storage().store::<Text>(KEY, Json(&x).into());
        x
    }

    pub fn remove() {
        storage().remove(KEY);
    }
}
