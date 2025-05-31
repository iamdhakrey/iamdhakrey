use std::os::linux::raw::stat;

use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    entities::{
        self, user::Entity as UserEntity, user::Model as UserModel,
    },
    response::GenericErrorResponse,
};

#[derive(Debug, Clone)]
pub struct UserCheck {
    pub id: Option<Uuid>,
    pub username: String,
    pub email: String,
}

pub async fn get_user_info(
    user: UserCheck,
    db: &DatabaseConnection,
) -> Result<UserModel, StatusCode> {
    // Fetch the user from the database
    match UserEntity::find()
        .filter(<entities::user::Entity as sea_orm::EntityTrait>::Column::Username.eq(user.username))
        .one(db)
        .await
    {
        Ok(Some(user)) => {
            let _user_by_id = UserEntity::find()
                .filter(<entities::user::Entity as sea_orm::EntityTrait>::Column::Id.eq(Some(user.id)))
                .one(db)
                .await
                .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
            if _user_by_id.is_some() {
                let _user_by_email = UserEntity::find()
                    .filter(<entities::user::Entity as sea_orm::EntityTrait>::Column::Email.eq(user.email.clone()))
                    .one(db)
                    .await
                    .map_err(|e| GenericErrorResponse::internal_error(e.to_string()));
                if _user_by_email.is_ok() {
                    Ok(user)
                } else {
                    Err(StatusCode::NOT_FOUND)
                }
            } else {
                return Err(StatusCode::NOT_FOUND);
            }
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
