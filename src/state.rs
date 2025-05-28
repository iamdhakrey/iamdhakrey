use std::sync::Arc;

use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}
