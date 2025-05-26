use std::{path::PathBuf, sync::Arc};

use directories::BaseDirs;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::models::AppConfig;

// pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
//     let config = load_config();
//     config
// });

pub static CONFIG: Lazy<Arc<RwLock<AppConfig>>> = Lazy::new(|| {
    let config = Arc::new(RwLock::new(load_config()));
    config
});
// pub static CONFIG: Lazy<Arc<RwLock<AppConfig>>> = Lazy::new(|| {
// let config = load_config();
// Arc::new(RwLock::new(config))
// });
fn config_path() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    let config_dir =
        base_dirs.config_dir().join("spendlite").join("api.toml");
    return config_dir;
}

/// Load the configuration from the config file
pub fn load_config() -> AppConfig {
    let base_dirs = BaseDirs::new().unwrap();

    let config_dir = config_path();

    // check if not exist then create the config file
    if !config_dir.exists() {
        std::fs::create_dir_all(base_dirs.config_dir().join("spendlite"))
            .unwrap();
        std::fs::write(
            config_dir.clone(),
            r#"database_url = "sqlite://spendlite.db"
level = "info"
host = "127.0.0.1"
port = 3000"#,
        )
        .unwrap();
    }

    let config: AppConfig =
        toml::de::from_str(&std::fs::read_to_string(config_dir).unwrap())
            .unwrap();

    return config;
}
