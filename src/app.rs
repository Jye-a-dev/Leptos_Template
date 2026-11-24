use crate::config::env::AppEnv;
use crate::router::AppRouter;
use crate::stores::app_store::AppStore;
use leptos::prelude::*;
use leptos_meta::{Meta, Title, provide_meta_context};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  let env = AppEnv::from_env();
  let _ = AppStore::provide(env);

  view! {
      <Title text="Leptos Template - Production Ready" />
      <Meta name="description" content="Production-ready Leptos 0.8 & Tailwind CSS v4 Frontend Template" />
      <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <AppRouter />
  }
}
