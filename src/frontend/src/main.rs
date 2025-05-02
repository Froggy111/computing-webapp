//! main.rs

mod app;
use app::body::Body;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(Body);
}
