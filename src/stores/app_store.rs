use crate::config::env::AppEnv;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct AppStore {
  pub env: RwSignal<AppEnv>,
  pub user_name: RwSignal<String>,
  pub notifications_count: RwSignal<usize>,
}

impl AppStore {
  pub fn new(env: AppEnv) -> Self {
    Self {
      env: RwSignal::new(env),
      user_name: RwSignal::new("Solo/Team Developer".to_string()),
      notifications_count: RwSignal::new(3),
    }
  }

  pub fn provide(env: AppEnv) -> Self {
    let store = Self::new(env);
    provide_context(store);
    store
  }

  pub fn use_store() -> Self {
    expect_context::<Self>()
  }
}
