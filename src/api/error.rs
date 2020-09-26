use http::StatusCode;
use serde::Deserialize;
use thiserror::Error as ThisError;
use yew::format::Text;
use yew::services::fetch::Response;

#[derive(Deserialize, Debug)]
pub struct Details {
    pub code: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
#[derive(ThisError, Debug)]
pub enum APIError {
    /// 400
    #[error("BadRequest {0:?}")]
    BadRequest(Details),
    /// 401
    #[error("Unauthorized {0:?}")]
    Unauthorized(Details),
    /// 403
    #[error("Forbidden")]
    Forbidden,
    /// 404
    #[error("Not Found")]
    NotFound,
    /// 500
    #[error("Internal Server Error")]
    InternalServerError,
    /// An unrecognised server error code/format
    #[error("Unknown Server Error")]
    UnknownError(StatusCode),
    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError(serde_json::Error),
    /// request error
    #[error("Http Request Error")]
    RequestError,
}

pub fn resolve<T>(response: Response<Text>) -> Result<T, APIError>
where
    for<'de> T: Deserialize<'de>,
{
    if let (meta, Ok(data)) = response.into_parts() {
        if meta.status.is_success() {
            serde_json::from_str(&data).map_err(|e| APIError::DeserializeError(e))
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
