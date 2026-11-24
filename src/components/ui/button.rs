use crate::utils::cn::cn;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonVariant {
  #[default]
  Primary,
  Secondary,
  Outline,
  Ghost,
}

#[component]
pub fn Button(
  #[prop(optional)] variant: ButtonVariant,
  #[prop(optional, into)] class: String,
  #[prop(optional)] disabled: Signal<bool>,
  #[prop(optional, into)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
  children: Children,
) -> impl IntoView {
  let variant_class = match variant {
    ButtonVariant::Primary => {
      "bg-indigo-600 text-white hover:bg-indigo-500 shadow-sm active:bg-indigo-700"
    }
    ButtonVariant::Secondary => {
      "bg-slate-800 text-slate-100 hover:bg-slate-700 active:bg-slate-900 border border-slate-700"
    }
    ButtonVariant::Outline => {
      "bg-transparent text-indigo-400 border border-indigo-500/40 hover:bg-indigo-950/30 active:bg-indigo-900/40"
    }
    ButtonVariant::Ghost => {
      "bg-transparent text-slate-300 hover:bg-slate-800/60 active:bg-slate-800"
    }
  };

  let base_class = "inline-flex items-center justify-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-indigo-500 disabled:pointer-events-none disabled:opacity-50 cursor-pointer";
  let computed_class = cn(&[base_class, variant_class, &class]);

  view! {
      <button
          type="button"
          class=computed_class
          disabled=disabled
          on:click=move |ev| {
              if let Some(cb) = on_click {
                  cb.run(ev);
              }
          }
      >
          {children()}
      </button>
  }
}
