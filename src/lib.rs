mod app;
pub use app::App;

#[cfg(target_arch = "wasm32")]
mod web;
