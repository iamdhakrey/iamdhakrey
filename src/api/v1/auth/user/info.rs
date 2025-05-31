use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    entities::{
        self, user::Entity as UserEntity, user::Model as UserModel,
    },
    response::GenericErrorResponse,
};

pub async fn get_user_info(
    username: String,
    db: &DatabaseConnection,
) -> Result<UserModel, GenericErrorResponse> {
    // Fetch the user from the database
    match UserEntity::find()
        .filter(<entities::user::Entity as sea_orm::EntityTrait>::Column::Username.eq(username))
        .one(db)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(GenericErrorResponse::not_found("User not found")),
        Err(e) => Err(GenericErrorResponse::internal_error(e.to_string())),
    }
}
