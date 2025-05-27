use sea_orm::ActiveModelBehavior;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::PrimaryKeyTrait;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, DeriveEntityModel,
)]
#[sea_orm(table_name = "user_profiles")]
pub struct Model {
    /// Unique identifier for the user profile
    /// link to the User model
    /// This is the primary key in the database.
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,

    /// Profile picture URL of the user
    /// This can be a link to an external image or a base64 encoded image.
    /// This can be used to display the user's profile picture in the UI.
    pub profile_picture: Option<String>,

    /// Display name of the user
    /// This can be different from the username.
    /// This can be used to display the user's name in the UI.
    pub display_name: Option<String>,

    /// Bio of the user
    pub bio: Option<String>,

    /// Location of the user
    pub location: Option<String>,

    /// Website URL of the user
    pub website: Option<String>,

    /// Social media links of the user
    pub social_links: Option<serde_json::Value>,
    /// Metadata for the user
    pub metadata: Option<serde_json::Value>,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::user::Entity")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

// // Implement the reverse relation for sea_orm
// impl Related<Entity> for super::user::Entity {
//     fn to() -> RelationDef {
//         Relation::User.def().rev()
//     }
// }
