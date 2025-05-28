use axum::{
    Form, Json,
    extract::{FromRequest, Request, rejection::FormRejection},
    response::Response,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidateJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = Response>,
{
    type Rejection = Response;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state)
            .await
            .map_err(|err| match err {
                FormRejection::BytesRejection(_) => Response::builder()
                    .status(400)
                    .body("Invalid form data".into())
                    .unwrap(),
                FormRejection::InvalidFormContentType(_) => {
                    Response::builder()
                        .status(415)
                        .body("Unsupported content type".into())
                        .unwrap()
                }
                FormRejection::FailedToDeserializeForm(
                    failed_to_deserialize_form,
                ) => todo!(),
                FormRejection::FailedToDeserializeFormBody(
                    failed_to_deserialize_form_body,
                ) => todo!(),
                _ => todo!(),
            })?;
        value.validate().map_err(|e| {
            Response::builder()
                .status(422)
                .body(format!("Validation error: {}", e).into())
                .unwrap()
        })?;
        Ok(ValidateJson(value))
    }
}
