use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExampleItem {
  pub id: u64,
  pub title: String,
  pub description: String,
  pub is_completed: bool,
}
