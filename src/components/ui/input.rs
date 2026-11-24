use crate::utils::cn::cn;
use leptos::prelude::*;

#[component]
pub fn Input(
  #[prop(into)] value: RwSignal<String>,
  #[prop(optional, into)] placeholder: String,
  #[prop(optional, into)] input_type: Option<String>,
  #[prop(optional, into)] class: String,
  #[prop(optional, into)] error: Signal<Option<String>>,
) -> impl IntoView {
  let type_attr = input_type.unwrap_or_else(|| "text".to_string());
  let base_class = "w-full rounded-lg border border-slate-700 bg-slate-900/90 px-3.5 py-2 text-sm text-slate-100 placeholder-slate-500 transition-colors focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500";
  let computed_class = cn(&[base_class, &class]);

  view! {
      <div class="flex w-full flex-col gap-1.5">
          <input
              type=type_attr
              class=computed_class
              placeholder=placeholder
              prop:value=move || value.get()
              on:input=move |ev| {
                  value.set(event_target_value(&ev));
              }
          />
          {move || {
              error.get().map(|err_msg| {
                  view! {
                      <p class="text-xs font-medium text-rose-400">{err_msg}</p>
                  }
              })
          }}
      </div>
  }
}
