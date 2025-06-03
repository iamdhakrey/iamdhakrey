use axum::{Extension, Json, http::StatusCode};

use crate::{
    api::v1::{spend::schema::SpendData, validators::ValidateJson},
    response::GenericResponse,
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
) -> Result<Json<SpendData>, GenericResponse<String>> {
    // Validate the spend data
    if data.amount <= 0.0 {
        return Err(GenericResponse::<String>::error(
            "Amount must be greater than zero".to_string(),
            StatusCode::BAD_REQUEST,
        ));
    }

    // Insert the spend data into the database
    // let db = &state.db;
    // let spend_id = insert_spend(data, db).await.map_err(|e| {
    //     GenericResponse::<String>::error(
    //         format!("Failed to create spend: {}", e),
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //     )
    // })?;

    // Return the created spend data with a 201 status code
    Ok(Json(SpendData { id: spend_id, ..data }))
}
