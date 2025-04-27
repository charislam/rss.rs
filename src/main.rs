use components::App;

mod auth;
mod components;
mod stores;
mod utils;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
