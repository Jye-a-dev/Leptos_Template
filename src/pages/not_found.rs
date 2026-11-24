use crate::components::ui::{Button, ButtonVariant};
use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
  view! {
      <div class="flex min-h-[60vh] flex-col items-center justify-center px-4 text-center">
          <span class="font-mono text-6xl font-extrabold text-indigo-500">"404"</span>
          <h1 class="mt-4 text-2xl font-bold tracking-tight text-white sm:text-3xl">"Trang không tìm thấy"</h1>
          <p class="mt-2 text-sm text-slate-400">"Đường dẫn bạn yêu cầu không tồn tại hoặc đã bị thay đổi."</p>
          <div class="mt-6">
              <a href="/">
                  <Button variant=ButtonVariant::Primary>
                      "Về trang chủ"
                  </Button>
              </a>
          </div>
      </div>
  }
}
