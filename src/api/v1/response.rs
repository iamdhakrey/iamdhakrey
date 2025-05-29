// generic api response types
use axum::{
    Json,
    http::{StatusCode, status},
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

pub struct GenericResponse<T>
where
    T: Serialize,
{
    pub inner: ApiResponse<T>,
    pub status_code: StatusCode,
}

impl<T> GenericResponse<T>
where
    T: Serialize,
{
    pub fn success(
        status_code: StatusCode,
        message: Option<String>,
        data: T,
        status: String,
    ) -> Self {
        Self {
            inner: ApiResponse {
                status: status,
                data: Some(data),
                message: message,
            },
            status_code: status_code,
        }
    }
    pub fn error(message: String, status_code: StatusCode) -> Self {
        Self {
            inner: ApiResponse {
                status: "error".to_string(),
                message: Some(message),
                data: None,
            },
            status_code,
        }
    }
}
impl<T> IntoResponse for GenericResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.status_code, Json(self.inner)).into_response()
    }
}

pub struct ValidationErrorResponse {
    pub status: String,
    pub errors: Vec<String>,
}
impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        let response: ApiResponse<Vec<String>> = ApiResponse {
            status: self.status,
            message: Some("Validation Failed".to_string()),
            data: Some(self.errors),
        };
        let status_code = StatusCode::UNPROCESSABLE_ENTITY;
        (status_code, Json(response)).into_response()
    }
}
