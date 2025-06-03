use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

lazy_static::lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
}
/// The date of the spend in ISO 8601 format
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SpendData {
    #[validate(range(
        min = 0.01,
        message = "Amount must be greater than zero"
    ))]
    pub amount: f64,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Category must be between 1 and 50 characters long"
    ))]
    pub category: String,

    #[validate(length(
        max = 200,
        message = "Description must be at most 200 characters long"
    ))]
    pub description: Option<String>,

    #[validate(regex(path = *DATE_REGEX, message = "Date must be in ISO 8601 format (YYYY-MM-DD)"))]
    pub date: String,

    #[validate(length(min = 1, max = 50))]
    pub payment_method: Option<String>,

    pub is_recurring: Option<bool>,

    #[validate(length(max = 5))]
    pub tags: Option<Vec<String>>,

    #[validate(length(max = 100))]
    pub location: Option<String>,

    #[validate(length(max = 100))]
    pub person: Option<String>,
}
