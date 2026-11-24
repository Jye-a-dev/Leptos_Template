# Leptos 0.8 + Tailwind CSS v4 Enterprise Template (CSR)

Template khởi tạo dự án Frontend chuẩn **Production-ready** bằng **Rust**, **Leptos 0.8+**, **WebAssembly (Wasm)** và **Tailwind CSS v4**. Được thiết kế tối ưu cho cả **Solo Developer** và quy trình làm việc phối hợp **Team** quy mô lớn (chỉ cần `git clone`, cấu hình `.env`, `npm install` là code ngay).

---

## 1. Bảng Tech Stack & Phiên bản Thực tế

| Công nghệ / Thư viện         | Phiên bản                                | Vai trò & Mục đích                                                                         |
| :--------------------------- | :--------------------------------------- | :----------------------------------------------------------------------------------------- |
| **Rust Toolchain**           | `stable-x86_64-pc-windows-gnu` (1.100.0) | Biên dịch Wasm với target `wasm32-unknown-unknown`                                         |
| **Trunk**                    | `v0.22.2`                                | Wasm web application bundler & dev server (Port 5000)                                      |
| **Leptos**                   | `0.8.23`                                 | Full-stack reactive web framework (Chế độ CSR - Client Side Rendering)                     |
| **Leptos Router**            | `0.8.17`                                 | Hệ thống định tuyến declarative lồng nhau (`<Router>`, `<Routes>`, `<Route>`, `<Outlet/>`) |
| **Leptos Meta**              | `0.8.7`                                  | Quản lý thẻ `<Title>`, `<Meta>` reactive theo trang                                        |
| **console_error_panic_hook** | `0.1.7`                                  | Bắt panic trong Rust và ghi stack trace trực tiếp ra trình duyệt DevTools                  |
| **gloo-net**                 | `0.6.0`                                  | HTTP Client bọc Web APIs (`fetch`) hỗ trợ async/await chuẩn Rust                           |
| **serde & serde_json**       | `1.0.229` / `1.0.154`                    | Serialization và deserialization dữ liệu JSON                                              |
| **validator**                | `0.21.0`                                 | Thư viện xác thực schema dữ liệu & form input typed                                        |
| **tw_merge**                 | `0.1.22`                                 | Xử lý gộp và giải quyết xung đột class Tailwind CSS an toàn                                |
| **lucide-leptos**            | `3.63.0`                                 | Bộ icon SVG nhẹ nhàng, sắc nét cho Leptos                                                  |
| **Tailwind CSS**             | `^4.1.18`                                | Framework CSS atomic thế hệ mới (@tailwindcss/cli)                                         |
| **Prettier & Plugin**        | `^3.5.3` / `^0.6.11`                     | Tự động định dạng code và sắp xếp class Tailwind                                           |
| **Husky & Lint-staged**      | `^9.1.7` / `^15.5.0`                     | Git hooks tự động format và kiểm tra trước khi commit                                      |
| **Commitlint**               | `^19.8.3`                                | Ép chuẩn Conventional Commits (`feat:`, `fix:`, `chore:`...)                               |

---

## 2. Yêu cầu Môi trường (Prerequisites)

1. **Rust Toolchain (Stable):**
   - Đảm bảo đã cài Rust (`rustc >= 1.80.0`).
   - Cài đặt target WebAssembly:
     ```bash
     rustup target add wasm32-unknown-unknown
     ```
   - Cài đặt components phục vụ format và lint:
     ```bash
     rustup component add rustfmt clippy
     ```
2. **Trunk CLI:**
   - Kiểm tra `trunk --version`. Nếu chưa có:
     ```bash
     cargo install trunk
     ```
3. **Node.js & NPM:**
   - Node.js LTS (phiên bản `>= 20`, khuyến nghị `22` theo file `.nvmrc`).
   - Quản lý qua nvm (tùy chọn): `nvm use`.

---

## 3. Hướng dẫn Khởi chạy Nhanh (Quick Start)

Chỉ với 4 bước đơn giản:

```bash
# 1. Clone repository
git clone <repository-url> leptos_template
cd leptos_template

# 2. Tạo file cấu hình môi trường từ template
cp .env.example .env

# 3. Cài đặt các công cụ Node.js & Git Hooks
npm install

# 4. Khởi chạy máy chủ phát triển (Dev Server)
npm run dev
```

Ứng dụng sẽ tự động khởi động tại: **[http://localhost:5000](http://localhost:5000)**.

---

## 4. Bảng Biến Môi trường (Environment Variables)

Hệ thống cấu hình được nạp compile-time và validate thông qua struct `AppEnv` tại `src/config/env.rs`:

| Tên biến       | Kiểu dữ liệu   | Giá trị mặc định            | Mô tả chi tiết                                                    |
| :------------- | :------------- | :-------------------------- | :---------------------------------------------------------------- |
| `PORT`         | `u16`          | `5000`                      | Cổng HTTP cho Trunk Dev Server và preview server                  |
| `APP_ENV`      | `String`       | `development`               | Môi trường chạy ứng dụng (`development`, `staging`, `production`) |
| `APP_URL`      | `String (URL)` | `http://localhost:5000`     | URL công khai của giao diện Frontend                              |
| `API_BASE_URL` | `String (URL)` | `http://localhost:8000/api` | Địa chỉ Backend REST API mà ứng dụng kết nối tới                  |

- **Cơ chế hoạt động:** `AppEnv::from_env()` sử dụng `option_env!` của Rust để đọc biến môi trường tại thời điểm biên dịch, tự động fallback an toàn nếu chưa set, sau đó validate bằng `validator::Validate` để cảnh báo sớm nếu giá trị không hợp lệ.

---

## 5. Cấu trúc Thư mục (Architecture)

Dự án áp dụng mô hình **Thin Pages + Feature-Based Modules**:

```plaintext
src/
├── assets/                     # Tài nguyên tĩnh: fonts, icons, images nội bộ (.gitkeep)
├── components/
│   ├── ui/                     # UI components dùng chung toàn hệ thống
│   │   ├── button.rs           # Button component đa biến thể (Primary, Secondary, Outline, Ghost)
│   │   ├── input.rs            # Input text component có reactive error message
│   │   └── mod.rs
│   └── mod.rs
├── config/
│   ├── env.rs                  # Typed environment struct AppEnv có validation
│   └── mod.rs
├── features/                   # Mô hình Feature-based (Mỗi module nghiệp vụ là 1 thư mục độc lập)
│   └── example/
│       ├── api/                # Hàm gọi API dành riêng cho feature (example_api.rs)
│       ├── components/         # Các component nội bộ của feature (example_card.rs)
│       ├── schemas/            # Struct dữ liệu form & luật validate (example_schema.rs)
│       ├── services/           # Reactive state service (ExampleService với RwSignal)
│       ├── types/              # DTOs và struct dữ liệu của feature (example_types.rs)
│       └── mod.rs
├── layouts/
│   ├── main_layout.rs          # Khung giao diện chính (Header, Outlet, Footer)
│   └── mod.rs
├── lib/
│   ├── http_client.rs          # Wrapper HTTP Client chuẩn hóa Base URL & Headers (gloo-net)
│   └── mod.rs
├── pages/                      # CHỈ chứa các trang định tuyến mỏng (Thin Pages)
│   ├── home_page.rs            # Trang chủ: nạp resource và nhúng ExampleCard
│   ├── not_found.rs            # Trang 404 Not Found
│   └── mod.rs
├── router/
│   └── mod.rs                  # Khai báo cây định tuyến tập trung (<Router>, <Routes>, <Route>)
├── stores/
│   ├── app_store.rs            # Global state (AppStore) cung cấp qua provide_context
│   └── mod.rs
├── styles/
│   └── global.css              # Entrypoint CSS với @import "tailwindcss"; (Tailwind v4)
├── types/
│   ├── api.rs                  # Kiểu dữ liệu API chung (ApiResponse<T>)
│   └── mod.rs
├── utils/
│   ├── cn.rs                   # Hàm tiện ích gộp class Tailwind CSS an toàn qua tw_merge
│   └── mod.rs
├── app.rs                      # Root component: MetaContext, AppStore, Router
└── main.rs                     # Entrypoint mount ứng dụng vào DOM (mount_to_body)
```

---

## 6. Quy chuẩn Code cho Team (Development Guidelines)

1. **Chuẩn hóa Module Path:**
   - Toàn bộ source code import tuyệt đối từ gốc crate: `crate::components::*`, `crate::features::*`, `crate::config::*`, `crate::utils::*`... Không dùng đường dẫn tương đối lộn xộn `super::super::`.
2. **Cú pháp Leptos 0.8+ CSR:**
   - Dùng `#[component]`, `view!`, `signal()`, `RwSignal`, `Memo`, `LocalResource`.
   - **Tuyệt đối cấm** cú pháp cũ của Leptos 0.5/0.6 (`cx: Scope`, `create_signal`, `create_effect`, `create_resource`).
3. **Form & Data Validation:**
   - Định nghĩa DTO kèm derive `#[derive(Validate)]` của crate `validator`.
   - Kiểm tra lỗi trước khi gửi dữ liệu lên Backend hoặc cập nhật state:
     ```rust
     if let Err(errors) = schema.validate() {
         // Xử lý thông báo lỗi theo từng field
     }
     ```
4. **Tailwind CSS v4 & Class Merging:**
   - Tạo style bằng atomic classes kết hợp helper `cn(&[&str])` để ghi đè class an toàn:
     ```rust
     let computed_class = cn(&[base_class, variant_class, &custom_class]);
     ```

---

## 7. Danh sách NPM Scripts

| Lệnh                 | Ý nghĩa & Thao tác thực hiện                                                                                     |
| :------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `npm run dev`        | Đọc `PORT` từ file `.env` (mặc định 5000) và khởi chạy `trunk serve`                                             |
| `npm run build`      | Biên dịch bundle Wasm tối ưu hóa cao cho Production (`trunk build --release`)                                    |
| `npm run preview`    | Chạy dev server trên bản build release tại cổng điều khiển bởi file `.env`                                       |
| `npm run type-check` | Kiểm tra kiểu và tính hợp lệ cú pháp Rust (`cargo check --target wasm32-unknown-unknown`)                        |
| `npm run lint`       | Quét linter nghiêm ngặt qua Clippy (`cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings`) |
| `npm run format`     | Tự động định dạng toàn bộ mã nguồn Rust (`cargo fmt`) và HTML/CSS/JSON/MD (`prettier`)                           |

---

## 8. Quy chuẩn Git Commit & Git Hooks

Dự án cài sẵn **Husky v9+**, **lint-staged** và **Commitlint** để đảm bảo chất lượng code và lịch sử commit đồng nhất:

- **Pre-commit Hook:** Tự động chạy `rustfmt` trên các file `.rs` đã staged và chạy `prettier` trên các file web/tài liệu.
- **Commit-msg Hook:** Kiểm tra thông điệp commit theo chuẩn **Conventional Commits**:
  - `feat: thêm chức năng mới`
  - `fix: sửa lỗi`
  - `chore: cập nhật dependencies, cấu hình`
  - `refactor: tái cấu trúc code`
  - `docs: cập nhật tài liệu`
  - `style: định dạng code`
