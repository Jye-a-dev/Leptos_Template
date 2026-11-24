use crate::features::example::types::example_types::ExampleItem;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct ExampleService {
  pub items: RwSignal<Vec<ExampleItem>>,
  pub search_query: RwSignal<String>,
}

impl ExampleService {
  pub fn new() -> Self {
    Self {
      items: RwSignal::new(Vec::new()),
      search_query: RwSignal::new(String::new()),
    }
  }

  pub fn provide() -> Self {
    let service = Self::new();
    provide_context(service);
    service
  }

  pub fn use_service() -> Self {
    expect_context::<Self>()
  }

  pub fn toggle_complete(&self, id: u64) {
    self.items.update(|list| {
      if let Some(item) = list.iter_mut().find(|it| it.id == id) {
        item.is_completed = !item.is_completed;
      }
    });
  }

  pub fn add_item(&self, title: String, description: String) {
    let next_id = self
      .items
      .with(|list| list.iter().map(|it| it.id).max().unwrap_or(0) + 1);
    self.items.update(|list| {
      list.push(ExampleItem {
        id: next_id,
        title,
        description,
        is_completed: false,
      });
    });
  }
}
