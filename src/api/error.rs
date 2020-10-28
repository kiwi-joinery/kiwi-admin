use http::StatusCode;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::str;
use yew::format::Binary;
use yew::services::fetch::Response;

#[derive(PartialEq, Deserialize, Debug, Clone)]
pub struct Details {
    pub code: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum APIError {
    BadRequest(Details),
    Unauthorized(Details),
    Forbidden,
    NotFound,
    InternalServerError,
    UnknownError(StatusCode),
    DeserializeError,
    RequestError,
}

impl std::error::Error for APIError {}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let m = match self {
            APIError::BadRequest(d) => d
                .description
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            APIError::Unauthorized(d) => match d.code.as_str() {
                "INCORRECT_CREDENTIALS" => "Incorrect credentials",
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

pub fn resolve<T>(response: Response<Binary>) -> Result<T, APIError>
where
    for<'de> T: Deserialize<'de>,
{
    if let (meta, Ok(bin)) = response.into_parts() {
        let data = str::from_utf8(&bin).map_err(|_| APIError::DeserializeError)?;
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
