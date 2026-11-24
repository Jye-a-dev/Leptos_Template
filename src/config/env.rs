use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct AppEnv {
  #[validate(range(min = 1, max = 65535, message = "PORT must be between 1 and 65535"))]
  pub port: u16,

  #[validate(length(min = 1, message = "APP_ENV must not be empty"))]
  pub app_env: String,

  #[validate(url(message = "APP_URL must be a valid URL"))]
  pub app_url: String,

  #[validate(url(message = "API_BASE_URL must be a valid URL"))]
  pub api_base_url: String,
}

impl AppEnv {
  pub fn from_env() -> Self {
    let port = option_env!("PORT")
      .and_then(|p| p.parse::<u16>().ok())
      .unwrap_or(5000);

    let app_env = option_env!("APP_ENV").unwrap_or("development").to_string();

    let app_url = option_env!("APP_URL")
      .unwrap_or("http://localhost:5000")
      .to_string();

    let api_base_url = option_env!("API_BASE_URL")
      .unwrap_or("http://localhost:8000/api")
      .to_string();

    let config = Self {
      port,
      app_env,
      app_url,
      api_base_url,
    };

    if let Err(err) = config.validate() {
      leptos::logging::warn!("Environment configuration validation warning: {:?}", err);
    }

    config
  }
}
