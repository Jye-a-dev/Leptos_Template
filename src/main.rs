#![allow(special_module_name)]

pub mod app;
pub mod components;
pub mod config;
pub mod features;
pub mod layouts;
#[allow(special_module_name)]
pub mod lib;
pub mod pages;
pub mod router;
pub mod stores;
pub mod types;
pub mod utils;

use app::App;
use leptos::prelude::*;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
