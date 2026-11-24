use crate::features::example::api::fetch_examples;
use crate::features::example::components::ExampleCard;
use crate::features::example::services::ExampleService;
use crate::lib::http_client::HttpClient;
use crate::stores::app_store::AppStore;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
  let store = AppStore::use_store();
  let service = ExampleService::provide();

  // Async LocalResource loading initial items
  let items_resource = LocalResource::new(move || {
    let env = store.env.get();
    async move {
      let client = HttpClient::new(&env);
      fetch_examples(&client).await
    }
  });

  // Populate service with loaded initial items
  Effect::new(move |_| {
    if let Some(Ok(fetched_items)) = items_resource.get() {
      service.items.set(fetched_items);
    }
  });

  view! {
      <div class="mx-auto max-w-5xl px-4 py-8 sm:px-6 lg:px-8">
          // Hero section
          <div class="mb-10 text-center">
              <div class="inline-flex items-center gap-2 rounded-full border border-indigo-500/30 bg-indigo-500/10 px-3.5 py-1 text-xs font-medium text-indigo-400">
                  <span class="inline-block h-1.5 w-1.5 rounded-full bg-indigo-400"></span>
                  <span>"Production-ready Rust Frontend"</span>
              </div>
              <h1 class="mt-4 text-3xl font-extrabold tracking-tight text-white sm:text-4xl lg:text-5xl">
                  "Leptos 0.8 + Tailwind CSS v4"
              </h1>
              <p class="mx-auto mt-3 max-w-2xl text-base text-slate-400 sm:text-lg">
                  "Thiết lập chuẩn hóa cho cả Solo & Team Developer: Thin Pages, Feature-based Modules, Typed Environment Validation và Fine-grained Reactivity."
              </p>
          </div>

          // Feature Card integration
          <ExampleCard />
      </div>
  }
}
