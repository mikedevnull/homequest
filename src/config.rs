use std::str::FromStr;

use sqlx::sqlite::SqliteConnectOptions;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    database_url: String,
    pub secretkey: Option<String>,
}

impl From<Config> for SqliteConnectOptions {
    fn from(value: Config) -> Self {
        sqlx::sqlite::SqliteConnectOptions::from_str(value.database_url.as_str()).unwrap()
    }
}

pub fn load_configuration() -> Result<Config, config::ConfigError> {
    let default_config = include_str!("../default_config.toml");
    let local_config_file_name = ".local.toml";
    let config_file = std::env::var("HOMEQUEST_CONFIG_FILE").unwrap_or("homequest_config".into());
    log::info!("{}", config_file);
    let settings = config::Config::builder()
        .add_source(config::File::from_str(
            default_config,
            config::FileFormat::Toml,
        ))
        .add_source(config::File::with_name(local_config_file_name).required(false))
        .add_source(config::File::with_name(config_file.as_str()).required(false))
        .add_source(config::Environment::with_prefix("HOMEQUEST").separator("_"))
        .build()?;
    settings.try_deserialize::<Config>()
}
