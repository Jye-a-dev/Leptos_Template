use crate::layouts::main_layout::MainLayout;
use crate::pages::home_page::HomePage;
use crate::pages::not_found::NotFound;
use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn AppRouter() -> impl IntoView {
  view! {
      <Router>
          <Routes fallback=|| view! { <NotFound /> }>
              <ParentRoute path=path!("/") view=MainLayout>
                  <Route path=path!("") view=HomePage />
              </ParentRoute>
          </Routes>
      </Router>
  }
}
