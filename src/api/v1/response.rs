use serde::Serialize;

#[derive(Serialize)]
pub struct UserCreateResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
