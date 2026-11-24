use crate::config::env::AppEnv;
use gloo_net::http::Request;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct HttpClient {
  base_url: String,
}

impl HttpClient {
  pub fn new(env: &AppEnv) -> Self {
    Self {
      base_url: env.api_base_url.trim_end_matches('/').to_string(),
    }
  }

  pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, String> {
    let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
    let response = Request::get(&url)
      .header("Accept", "application/json")
      .send()
      .await
      .map_err(|e| format!("HTTP GET failed: {e}"))?;

    if !response.ok() {
      return Err(format!("Server returned HTTP {}", response.status()));
    }

    response
      .json::<T>()
      .await
      .map_err(|e| format!("JSON deserialization failed: {e}"))
  }

  pub async fn post<B: Serialize, T: DeserializeOwned>(
    &self,
    endpoint: &str,
    body: &B,
  ) -> Result<T, String> {
    let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));
    let response = Request::post(&url)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .json(body)
      .map_err(|e| format!("Failed to serialize body: {e}"))?
      .send()
      .await
      .map_err(|e| format!("HTTP POST failed: {e}"))?;

    if !response.ok() {
      return Err(format!("Server returned HTTP {}", response.status()));
    }

    response
      .json::<T>()
      .await
      .map_err(|e| format!("JSON deserialization failed: {e}"))
  }
}
