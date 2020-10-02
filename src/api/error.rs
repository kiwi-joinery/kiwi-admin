use http::StatusCode;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use yew::format::Text;
use yew::services::fetch::Response;

#[derive(PartialEq, Deserialize, Debug, Clone)]
pub struct Details {
    pub code: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum APIError {
    /// 400
    BadRequest(Details),
    /// 401
    Unauthorized(Details),
    /// 403
    Forbidden,
    /// 404
    NotFound,
    /// 500
    InternalServerError,
    /// An unrecognised server error code/format
    UnknownError(StatusCode),
    /// serde deserialize error
    DeserializeError,
    /// request error
    RequestError,
}
impl std::error::Error for APIError {}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let m = match self {
            APIError::BadRequest(d) => format!(
                "Bad request: {}",
                d.description.as_ref().unwrap_or(&"".to_string())
            ),
            APIError::Unauthorized(d) => match d.code.as_str() {
                "INCORRECT_CREDENTIALS" => "Incorrect username or password",
                "MISSING_CREDENTIALS" => "Request missing username or password",
                _ => "Unauthorized",
            }
            .to_string(),
            APIError::Forbidden => "Forbidden".to_string(),
            APIError::NotFound => "Resource not found".to_string(),
            APIError::InternalServerError => "Internal server error".to_string(),
            APIError::UnknownError(c) => format!("Unknown Error (HTTP {})", c.as_u16()),
            APIError::DeserializeError => "Could not parse server response".to_string(),
            APIError::RequestError => "Request failed, please check your connection".to_string(),
        };
        write!(f, "{}", m)
    }
}

pub fn resolve<T>(response: Response<Text>) -> Result<T, APIError>
where
    for<'de> T: Deserialize<'de>,
{
    if let (meta, Ok(data)) = response.into_parts() {
        if meta.status.is_success() {
            serde_json::from_str(&data).map_err(|_| APIError::DeserializeError)
        } else {
            let details: Option<Details> = serde_json::from_str(&data).ok();
            Err(match meta.status {
                StatusCode::BAD_REQUEST if details.is_some() => {
                    APIError::BadRequest(details.unwrap())
                }
                StatusCode::UNAUTHORIZED if details.is_some() => {
                    APIError::Unauthorized(details.unwrap())
                }
                StatusCode::FORBIDDEN => APIError::Forbidden,
                StatusCode::NOT_FOUND => APIError::NotFound,
                StatusCode::INTERNAL_SERVER_ERROR => APIError::InternalServerError,
                _ => APIError::UnknownError(meta.status),
            })
        }
    } else {
        Err(APIError::RequestError)
    }
}
