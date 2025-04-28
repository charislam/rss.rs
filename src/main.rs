use std::sync::LazyLock;

use components::App;
use managers::user::UserManager;

mod auth;
mod components;
mod managers;
mod stores;
mod utils;

pub(crate) static USER_MANAGER: LazyLock<UserManager> = LazyLock::new(UserManager::new);

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
