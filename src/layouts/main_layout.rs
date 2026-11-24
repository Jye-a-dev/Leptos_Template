use crate::stores::app_store::AppStore;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use lucide_leptos::{Code, LayoutDashboard};

#[component]
pub fn MainLayout() -> impl IntoView {
  let store = AppStore::use_store();

  view! {
      <div class="flex min-h-screen flex-col bg-slate-950 text-slate-100">
          // Header
          <header class="sticky top-0 z-40 border-b border-slate-800/80 bg-slate-950/80 backdrop-blur-md">
              <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
                  <div class="flex items-center gap-3">
                      <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-indigo-600/20 text-indigo-400 ring-1 ring-indigo-500/40">
                          <Code size=20 />
                      </div>
                      <div>
                          <span class="text-base font-semibold tracking-tight text-white">"Leptos Template"</span>
                          <span class="ml-2 rounded-md bg-indigo-500/10 px-2 py-0.5 text-xs font-medium text-indigo-400 ring-1 ring-indigo-500/30">
                              "Production-Ready"
                          </span>
                      </div>
                  </div>
                  <div class="flex items-center gap-4 text-sm text-slate-400">
                      <div class="flex items-center gap-2">
                          <span class="inline-block h-2 w-2 rounded-full bg-emerald-400"></span>
                          <span class="text-xs font-medium text-slate-300">
                              {move || store.env.get().app_env}
                          </span>
                      </div>
                      <span class="hidden text-slate-600 sm:inline">"|"</span>
                      <div class="hidden text-xs text-slate-400 sm:inline">
                          "Port: " <span class="font-mono text-indigo-300">{move || store.env.get().port}</span>
                      </div>
                  </div>
              </div>
          </header>

          // Main Content with Nested Routes Outlet
          <main class="flex-1">
              <Outlet />
          </main>

          // Footer
          <footer class="border-t border-slate-900 bg-slate-950 py-6 text-center text-xs text-slate-500">
              <div class="mx-auto flex max-w-7xl flex-col items-center justify-between gap-3 px-4 sm:flex-row sm:px-6 lg:px-8">
                  <div class="flex items-center gap-2">
                      <LayoutDashboard size=16 />
                      <span>"Leptos 0.8 CSR Template"</span>
                  </div>
                  <div>
                      "Built with Rust, WebAssembly & Tailwind CSS v4"
                  </div>
              </div>
          </footer>
      </div>
  }
}
