use axum::{Extension, Json};

use crate::{
    api::v1::{
        spend::{add::add_spend, schema::SpendData},
        validators::ValidateJson,
    },
    entities::user::ActiveModel as UserActiveModel,
    response::GenericErrorResponse,
    state::AppState,
};

#[utoipa::path(
    post,
    summary = "Create a new spend",
    description = "Create a new spend entry in the application",
    tag = "Spend",
    path = "/spend",
    request_body = SpendData,
    responses(
        (status = 201, description = "Spend created successfully"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
)]
pub async fn create_spend(
    Extension(state): Extension<AppState>, // Database connection state
    ValidateJson(data): ValidateJson<SpendData>,
    user: UserActiveModel,
) -> Result<Json<String>, GenericErrorResponse> {
    let user_id = user.id.unwrap();
    add_spend(user_id, Json(data), &state.db).await
}
