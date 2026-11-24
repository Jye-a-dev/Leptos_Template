use crate::components::ui::{Button, ButtonVariant, Input};
use crate::features::example::schemas::ExampleFormSchema;
use crate::features::example::services::ExampleService;
use crate::utils::cn::cn;
use leptos::prelude::*;
use lucide_leptos::{Check, Code};
use validator::Validate;

#[component]
pub fn ExampleCard() -> impl IntoView {
  let service = ExampleService::use_service();

  // Local form state using fine-grained RwSignals
  let title_input = RwSignal::new(String::new());
  let desc_input = RwSignal::new(String::new());
  let title_error = RwSignal::new(None::<String>);
  let desc_error = RwSignal::new(None::<String>);

  // Counter signal demonstrating fine-grained reactivity
  let (counter, set_counter) = signal(0);

  // Derived memo for completed items count
  let completed_count = Memo::new(move |_| {
    service
      .items
      .with(|list| list.iter().filter(|i| i.is_completed).count())
  });

  let total_count = Memo::new(move |_| service.items.with(|list| list.len()));

  let on_submit = move |_| {
    let form_data = ExampleFormSchema {
      title: title_input.get().trim().to_string(),
      description: desc_input.get().trim().to_string(),
    };

    title_error.set(None);
    desc_error.set(None);

    match form_data.validate() {
      Ok(_) => {
        service.add_item(form_data.title, form_data.description);
        title_input.set(String::new());
        desc_input.set(String::new());
        set_counter.update(|c| *c += 1);
      }
      Err(validation_errors) => {
        for (field, errors) in validation_errors.field_errors() {
          let first_msg = errors
            .first()
            .and_then(|e| e.message.as_ref())
            .map(|m| m.to_string())
            .unwrap_or_else(|| "Giá trị không hợp lệ".to_string());

          if field == "title" {
            title_error.set(Some(first_msg));
          } else if field == "description" {
            desc_error.set(Some(first_msg));
          }
        }
      }
    }
  };

  view! {
      <div class="rounded-2xl border border-slate-800 bg-slate-900/50 p-6 shadow-xl backdrop-blur-sm sm:p-8">
          <div class="mb-6 flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
              <div>
                  <h2 class="text-xl font-bold tracking-tight text-white sm:text-2xl">
                      "Feature Workflow: Task Tracker"
                  </h2>
                  <p class="text-sm text-slate-400">
                      "Minh họa kiến trúc Feature-Based, validation qua validator crate, và Signal reactivity."
                  </p>
              </div>
              <div class="flex items-center gap-2">
                  <span class="rounded-lg bg-indigo-500/10 px-3 py-1 font-mono text-xs font-semibold text-indigo-400 ring-1 ring-indigo-500/30">
                      {move || format!("Đã thêm: {}", counter.get())}
                  </span>
                  <span class="rounded-lg bg-emerald-500/10 px-3 py-1 font-mono text-xs font-semibold text-emerald-400 ring-1 ring-emerald-500/30">
                      {move || format!("{}/{} Hoàn thành", completed_count.get(), total_count.get())}
                  </span>
              </div>
          </div>

          // Form creation
          <div class="mb-8 rounded-xl border border-slate-800/80 bg-slate-950/60 p-4 sm:p-5">
              <h3 class="mb-3 text-sm font-semibold text-slate-200">"Thêm tính năng mới"</h3>
              <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <Input
                      value=title_input
                      placeholder="Tiêu đề (tối thiểu 3 ký tự)..."
                      error=Signal::derive(move || title_error.get())
                  />
                  <Input
                      value=desc_input
                      placeholder="Mô tả (tối thiểu 5 ký tự)..."
                      error=Signal::derive(move || desc_error.get())
                  />
              </div>
              <div class="mt-4 flex justify-end">
                  <Button
                      variant=ButtonVariant::Primary
                      on_click=Callback::new(on_submit)
                  >
                      <Code size=16 />
                      <span>"Xác thực & Thêm"</span>
                  </Button>
              </div>
          </div>

          // List of items
          <div class="space-y-3">
              <h3 class="text-sm font-semibold text-slate-300">"Danh sách hạng mục"</h3>
              {move || {
                  let items_list = service.items.get();
                  if items_list.is_empty() {
                      view! {
                          <p class="py-6 text-center text-sm text-slate-500">"Chưa có hạng mục nào."</p>
                      }.into_any()
                  } else {
                      items_list.into_iter().map(|item| {
                          let item_id = item.id;
                          let is_completed = item.is_completed;
                          let card_class = cn(&[
                              "flex items-start justify-between gap-4 rounded-xl border p-4 transition-all duration-150",
                              if is_completed {
                                  "border-emerald-500/30 bg-emerald-950/10 text-slate-400"
                              } else {
                                  "border-slate-800 bg-slate-900/30 text-slate-200 hover:border-slate-700"
                              }
                          ]);

                          view! {
                              <div class=card_class>
                                  <div class="flex-1">
                                      <div class="flex items-center gap-2">
                                          <span class="font-mono text-xs text-slate-500">
                                              {format!("#{item_id}")}
                                          </span>
                                          <h4 class=cn(&[
                                              "text-sm font-medium",
                                              if is_completed { "line-through text-slate-500" } else { "text-white" }
                                          ])>
                                              {item.title}
                                          </h4>
                                      </div>
                                      <p class="mt-1 text-xs text-slate-400">
                                          {item.description}
                                      </p>
                                  </div>
                                  <Button
                                      variant=if is_completed { ButtonVariant::Secondary } else { ButtonVariant::Outline }
                                      class="shrink-0 text-xs py-1.5 px-3"
                                      on_click=Callback::new(move |_| {
                                          service.toggle_complete(item_id);
                                      })
                                  >
                                      <Check size=14 />
                                      <span>{if is_completed { "Hoàn tất" } else { "Đánh dấu xong" }}</span>
                                  </Button>
                              </div>
                          }
                      }).collect::<Vec<_>>().into_any()
                  }
              }}
          </div>
      </div>
  }
}
