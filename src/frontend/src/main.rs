//! main.rs

mod app;
use app::button::Button;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(Button);
}
