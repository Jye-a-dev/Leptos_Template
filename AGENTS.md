# Leptos Template Architectural Standards & Guidelines

This repository follows production-ready Leptos 0.8+ (CSR) with Tailwind CSS v4 and Rust 2024 edition conventions.

## 1. Modern Leptos Reactive Primitives

- BẮT BUỘC dùng modern Leptos API: `#[component]`, `view!`, fine-grained signals (`signal()`, `RwSignal`, `Memo`, `Effect`), `LocalResource` / `Resource`.
- BẮT BUỘC dùng `leptos_router` (`<Router>`, `<Routes>`, `<Route>`, `<ParentRoute>`, `<Outlet />`, `path!()` macro).
- BẮT BUỘC dùng `leptos_meta` (`provide_meta_context()`, `<Title>`, `<Meta>`).
- TUYỆT ĐỐI CẤM cú pháp Leptos 0.5/0.6 cũ: `cx: Scope`, `create_signal`, `create_effect`, `create_memo`, `create_resource`.

## 2. Module Path & Structure Conventions

- Tất cả module import phải dùng đường dẫn gốc nhất quán: `crate::components::*`, `crate::features::*`, `crate::config::*`, `crate::utils::*`, `crate::stores::*`.
- Thin Pages Architecture: Thư mục `src/pages/` CHỈ chứa component trang định tuyến mỏng; toàn bộ logic nghiệp vụ, schema, API call và sub-components phải đặt trong `src/features/<feature_name>/`.
- UI Components: Thư mục `src/components/ui/` chỉ chứa component giao diện tái sử dụng dùng chung toàn ứng dụng.

## 3. Styling & Tooling Standards

- Tailwind CSS v4: `@import "tailwindcss";` trong `src/styles/global.css`, nhúng qua `<link data-trunk rel="tailwind-css" href="src/styles/global.css" />` trong `index.html`. Không sử dụng `tailwind.config.js`.
- Class merging: Dùng helper `crate::utils::cn::cn(&[&str])` xây dựng trên `tw_merge`.
- Environment Configuration: Validate compile-time (option_env!) với fallback an toàn từ `.env.example` bằng `validator::Validate` trong `crate::config::env`.

## 4. Quality Guardrails & Git Commits

- Toolchain: Target `wasm32-unknown-unknown`, `rustfmt`, `clippy` (`-D warnings`).
- Formatting: `cargo fmt` cho `.rs` (edition 2024); `prettier` + `prettier-plugin-tailwindcss` cho web assets.
- Commits: Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `perf:`).
