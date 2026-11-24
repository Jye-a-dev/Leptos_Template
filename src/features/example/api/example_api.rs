use crate::features::example::types::example_types::ExampleItem;
use crate::lib::http_client::HttpClient;

/// Mock / API call showcasing async data fetching with HttpClient
pub async fn fetch_examples(_client: &HttpClient) -> Result<Vec<ExampleItem>, String> {
  Ok(vec![
    ExampleItem {
      id: 1,
      title: "Fine-grained Signals".to_string(),
      description: "Sử dụng signal() & RwSignal cập nhật DOM tối ưu từng node riêng biệt."
        .to_string(),
      is_completed: true,
    },
    ExampleItem {
      id: 2,
      title: "Thin Pages Architecture".to_string(),
      description: "Trang định tuyến mỏng, chuyển toàn bộ logic nghiệp vụ vào src/features/."
        .to_string(),
      is_completed: true,
    },
    ExampleItem {
      id: 3,
      title: "Tailwind CSS v4 Engine".to_string(),
      description: "Tích hợp @import 'tailwindcss' và Trunk data-trunk pipeline mượt mà."
        .to_string(),
      is_completed: false,
    },
  ])
}
