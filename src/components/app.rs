use leptos::prelude::*;
use leptos_router::{components::{Route, Router, Routes}, hooks::use_navigate, path};
use reactive_stores::Store;

use crate::components::{AuthPage, HomePage};
use crate::stores::user::{UserStoreStoreFields, UserStore};

#[component]
pub(crate) fn App() -> impl IntoView {
    let user_store = UserStore::new();
    provide_context(Store::new(user_store));

    view! {
        <Router>
            <Navigate />
            <Routes fallback=HomePage>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/auth") view=AuthPage />
            </Routes>
        </Router>
    }
}

fn Navigate() -> impl IntoView {
    let user_store = expect_context::<Store<UserStore>>();
    let logged_in = user_store.data().read().is_some();

    if !logged_in {
        let navigate = use_navigate();
        navigate ("/auth", Default::default());
    }

    ()
}
