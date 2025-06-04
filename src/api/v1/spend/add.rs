// add spend data
use axum::Json;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    api::v1::spend::schema::SpendData,
    entities::spend::ActiveModel as SpendActiveModel,
    response::GenericErrorResponse,
};

pub async fn add_spend(
    user_id: uuid::Uuid, // User ID for whom the spend is being created
    spend_data: Json<SpendData>, // Spend data from the request
    db: &DatabaseConnection, // Database connection
) -> Result<Json<String>, GenericErrorResponse> {
    // Validate the spend data
    if spend_data.amount <= 0.0 {
        return Err(GenericErrorResponse::bad_request(
            "Amount must be greater than zero".to_string(),
            Some("Amount must be greater than zero".to_string()),
        ));
    }

    // Insert the spend data into the database
    let spend_id = SpendActiveModel {
        id: sea_orm::ActiveValue::Set(uuid::Uuid::new_v4()),
        user_id: sea_orm::ActiveValue::Set(user_id),
        category: sea_orm::ActiveValue::Set(spend_data.category.clone()),
        payment_method: sea_orm::ActiveValue::Set(
            spend_data.payment_method.clone(),
        ),
        is_recurring: sea_orm::ActiveValue::Set(spend_data.is_recurring),
        tags: sea_orm::ActiveValue::Set(
            spend_data
                .tags
                .as_ref()
                .map(|t| serde_json::to_string(t).unwrap()),
        ),
        location: sea_orm::ActiveValue::Set(spend_data.location.clone()),
        person: sea_orm::ActiveValue::Set(spend_data.person.clone()),

        amount: sea_orm::ActiveValue::Set(
            Decimal::from_f64(spend_data.amount).unwrap(),
        ),
        description: sea_orm::ActiveValue::Set(
            spend_data.description.clone(),
        ),
        date: sea_orm::ActiveValue::Set(
            NaiveDate::parse_from_str(&spend_data.date, "%Y-%m-%d")
                .unwrap(),
        ),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|e| {
        GenericErrorResponse::internal_error(format!(
            "Failed to create spend: {}",
            e
        ))
    })?;
    // Return the created spend data with a 201 status code
    Ok(Json(format!(
        "Spend created successfully with ID: {}",
        spend_id.id
    )))
}
