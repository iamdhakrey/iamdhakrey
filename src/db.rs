pub async fn connect(db_url: &str) -> sea_orm::DatabaseConnection {
    sea_orm::Database::connect(db_url)
        .await
        .expect("Failed to connect to the database")
}
