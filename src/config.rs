use directories::BaseDirs;
use once_cell::sync::Lazy;

use crate::models::AppConfig;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| load_config());

/// Load the configuration from the config file
pub fn load_config() -> AppConfig {
    let base_dirs = BaseDirs::new().unwrap();

    let config_dir =
        base_dirs.config_dir().join("spendlite").join("api.toml");

    // check if not exist then create the config file
    if !config_dir.exists() {
        std::fs::create_dir_all(base_dirs.config_dir().join("spendlite"))
            .unwrap();
        std::fs::write(
            config_dir.clone(),
            r#"
url = "sqlite://spendlite.db"
level = "info"
"#,
        )
        .unwrap();
    }

    let config: AppConfig =
        toml::de::from_str(&std::fs::read_to_string(config_dir).unwrap())
            .unwrap();

    return config;
}
