use super::info::get_user_info;
use crate::{
    api::v1::auth::{schema::SignUpData, user::info::UserCheck},
    entities::user::ActiveModel as UserActiveModel,
    entities::user::Entity,
    response::GenericErrorResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::{error, info};
pub async fn create_user(
    user: SignUpData,
    db: &DatabaseConnection,
) -> Result<UserActiveModel, GenericErrorResponse> {
    // Check if the user already exists
    let _check_user = UserCheck {
        id: None,
        username: user.username.clone(),
        email: user.email.clone(),
    };
    let existing_user = get_user_info(_check_user, db).await;
    if existing_user.is_ok() {
        return Err(GenericErrorResponse::conflict(
            "User already exists".to_string(),
        ));
    }
    info!("Creating user: {:?}", user.username);

    let _full_name = format!(
        "{} {}",
        user.first_name.clone().unwrap_or_default(),
        user.last_name.clone().unwrap_or_default()
    );

    // Create a new user model
    let hash_password = crate::api::v1::auth::user::hash::hash_password(
        &user.password.clone(),
    )
    .map_err(|_| {
        GenericErrorResponse::internal_error(
            "Failed to hash password".to_string(),
        )
    })?;
    let _uuid = uuid::Uuid::new_v4();
    let new_user = UserActiveModel {
        id: sea_orm::ActiveValue::Set(_uuid),
        username: sea_orm::ActiveValue::Set(user.username.clone()),
        email: sea_orm::ActiveValue::Set(user.email.clone()),
        full_name: sea_orm::ActiveValue::Set(Some(_full_name)),
        password_hash: sea_orm::ActiveValue::Set(hash_password),
        ..Default::default()
    };
    let user = new_user.clone();
    // Insert the new user into the database
    let _user = Entity::insert(new_user).exec(db).await.map_err(|e| {
        error!("Database insert error: {:?}", e);
        GenericErrorResponse::internal_error(
            "Failed to create user".to_string(),
        )
    })?;
    info!("User created successfully: {:?}", user.username);
    Ok(user)
}
