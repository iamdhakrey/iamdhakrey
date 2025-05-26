use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub log_level: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}
