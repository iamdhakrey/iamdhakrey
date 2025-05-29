use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use validator::Validate;

use crate::api::v1::response::{
    ErrorResponse, GenericResponse, ValidationErrorResponse,
};

pub struct ValidateJson<T>(pub T);

impl<T> Deref for ValidateJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidateJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Json(value) =
            Json::<T>::from_request(req, state).await.map_err(|err| {
                GenericResponse::<ErrorResponse>::error(
                    format!("Invalid JSON: {}", err),
                    StatusCode::BAD_REQUEST,
                )
                .into_response()
            })?;
        match value.validate() {
            Ok(_) => {}
            Err(e) => {
                let errors = e
                    .field_errors()
                    .iter()
                    .map(|(field, errs)| {
                        let messages = errs
                            .iter()
                            .map(|e| {
                                e.message
                                    .clone()
                                    .unwrap_or_default()
                                    .to_string()
                            })
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{field}: {messages}")
                    })
                    .collect::<Vec<_>>();
                // .join("; ");
                return Err(ValidationErrorResponse {
                    status: "error".to_string(),
                    errors: errors,
                }
                .into_response());
            }
        }
        Ok(ValidateJson(value))
    }
}
