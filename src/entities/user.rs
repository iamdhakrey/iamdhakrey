use sea_orm::ActiveModelBehavior;
use sea_orm::DerivePrimaryKey;
use sea_orm::EnumIter;
use sea_orm::PrimaryKeyTrait;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, RelationTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// Unique identifier for the user uuid
    /// This is the primary key in the database.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Username of the user
    /// This should be unique and validated.
    #[sea_orm(unique)]
    pub username: String,

    /// Email address of the user
    /// This should be unique and validated.
    #[sea_orm(unique)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: String,

    /// Full name of the user
    pub full_name: Option<String>,

    /// Password hash of the user
    /// This should be securely hashed and stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,

    /// Whether the user is active
    /// This can be used to disable accounts without deleting them.
    pub is_active: bool,

    /// Whether the user is an admin
    /// This can be used to grant special permissions.
    pub is_admin: bool,

    /// Creation timestamp of the user
    pub created_at: chrono::NaiveDateTime,

    /// Last updated timestamp of the user
    pub updated_at: chrono::NaiveDateTime,

    /// Last login timestamp of the user
    /// This can be used for tracking user activity.
    pub last_login: Option<chrono::NaiveDateTime>,

    /// Optional metadata for the user
    pub metadata: Option<serde_json::Value>,

    /// Optional phone number of the user
    /// This can be used for two-factor authentication or notifications.
    pub phone_number: Option<String>,

    /// Regitration Version of the user
    /// This can be used to track the version of the application the user registered with.
    /// This can be useful for feature flags or migrations.
    /// This can also provide extra features for users who registered with a specific version.
    pub registration_version: Option<String>,

    /// Store the Login Provider of the user
    /// This can be used to track the provider the user used to login.
    pub login_provider: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}

// Required by SeaORM's DeriveEntityModel macro
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_profile::Entity")]
    UserProfile,
}

impl Related<super::user_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserProfile.def()
    }
}
