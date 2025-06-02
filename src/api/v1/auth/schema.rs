use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct SignUpData {
    /// Username field with validation
    #[validate(length(
        min = 5,
        max = 20,
        message = "Username must be between 5 and 20 characters long"
    ))]
    pub username: String,

    /// Email field with validation
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(
        min = 8,
        message = "Password must be at least 8 characters long"
    ))]

    /// Password field with validation  
    pub password: String,
    #[validate(length(
        min = 2,
        max = 50,
        message = "First name must be between 2 and 50 characters long"
    ))]
    pub first_name: Option<String>,

    // Optional last name field
    #[validate(length(
        min = 2,
        max = 50,
        message = "Last name must be between 2 and 50 characters long"
    ))]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

// Define the Auth struct to represent the authentication information
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct SignInData {
    /// Username or email field with validation
    #[validate(length(
        min = 5,
        message = "Username must be at least 5 characters long"
    ))]
    pub username: String,
    #[validate(length(min = 6))]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: String,
}
