use super::info::get_user_info;
use crate::{
    api::v1::auth::schema::SignUpData, entities::prelude::User,
    entities::user::Entity, response::GenericErrorResponse,
};
use sea_orm::DatabaseConnection;
pub async fn create_user(
    user: SignUpData,
    db: &DatabaseConnection,
) -> Result<User, GenericErrorResponse> {
    // Check if the user already exists
    let username = user.username.clone();
    let existing_user = get_user_info(username, db).await;
    if existing_user.is_ok() {
        return Err(GenericErrorResponse::conflict(
            "User already exists".to_string(),
        ));
    }

    todo!("Implement user creation logic");
}
