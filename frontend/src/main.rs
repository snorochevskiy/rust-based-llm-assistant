mod components;
mod api;
mod entity;
fn main() {
    console_error_panic_hook::set_once();
    leptos::logging::log!("Starting...");
    leptos::mount::mount_to_body(components::app::App)
}
