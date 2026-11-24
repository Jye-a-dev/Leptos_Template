use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Validate)]
pub struct ExampleFormSchema {
  #[validate(length(min = 3, message = "Tiêu đề phải có ít nhất 3 ký tự"))]
  pub title: String,

  #[validate(length(min = 5, message = "Mô tả phải có ít nhất 5 ký tự"))]
  pub description: String,
}
