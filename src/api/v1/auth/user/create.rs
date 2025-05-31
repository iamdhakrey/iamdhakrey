pub async fn create_user(
    user: CreateUser,
    db: &DatabaseConnection,
) -> Result<User, ApiError> {
    let user = User::insert(user).exec(db).await?;
    Ok(user)
}
