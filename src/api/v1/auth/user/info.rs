use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;
use uuid::Uuid;

use crate::entities::{
    self, user::Entity as UserEntity, user::Model as UserModel,
};

pub struct UserCheck {
    pub id: Option<Uuid>,
    pub username: String,
    pub email: String,
}

pub async fn get_user_info(
    user_check: UserCheck,
    db: &DatabaseConnection,
) -> Result<UserModel, StatusCode> {
    // Build base query
    let mut query = UserEntity::find().filter(
        entities::user::Column::Username.eq(user_check.username.clone()),
    );
    // .filter(
    //     entities::user::Column::Email.eq(user_check.email.clone()),
    // );

    // Optional ID filter
    if let Some(id) = user_check.id {
        query = query.filter(entities::user::Column::Id.eq(id));
    }

    // Execute query
    match query.one(db).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => {
            let email_query = UserEntity::find()
                .filter(
                    entities::user::Column::Email
                        .eq(user_check.email.clone()),
                )
                .one(db)
                .await;
            if let Ok(Some(_)) = email_query {
                error!(
                    "User with email {} already exists",
                    user_check.email
                );
                return Err(StatusCode::CONFLICT);
            }
            error!("User with username {} not found", user_check.username);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Database query failed: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
