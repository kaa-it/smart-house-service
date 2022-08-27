use serde_aux::field_attributes::deserialize_number_from_string;
use secrecy::{ExposeSecret, Secret};

/// Supported environments for application
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} id not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

/// All settings
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

/// Application settings section
#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

/// Database settings section
#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl Settings {
    pub fn new() -> Result<Settings, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("configuration");

        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        let settings = config::Config::builder()
            .add_source(config::File::from(configuration_directory.join("base")).required(true))
            .add_source(
                config::File::from(configuration_directory.join(environment.as_str()))
                    .required(true),
            )
            // Append settings from environment variables (with prefix APP and '__' as separator`)
            // For example, `APP__APPLICATION__PORT=5001` will setup `Settings.application.port`
            .add_source(config::Environment::with_prefix("app").separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}

impl DatabaseSettings {
    pub fn uri(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/admin?authSource={}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
    }
}