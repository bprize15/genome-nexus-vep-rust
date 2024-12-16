use secrecy::SecretString;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub vep: VepSettings
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct VepSettings {
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: SecretString,
    pub forks: u16,
}

pub enum Environment {
    Dev,
    Prod
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse app environment");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join(environment_filename)))
        .build()?;
    settings.try_deserialize()
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
           Environment::Dev => "dev",
           Environment::Prod => "prod" 
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Environment::Dev),
            "prod" => Ok(Environment::Prod),
            other => Err(format!(
                "{} is not a supported environment. \
                User either 'dev' or 'prod'.",
                other
            ))
        }
    }
}
