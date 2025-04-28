use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_navigate,
    path,
};
use reactive_stores::Store;

use crate::stores::user::{UserStore, UserStoreStoreFields};
use crate::{
    USER_MANAGER,
    components::{AuthPage, HomePage},
};

#[component]
pub(crate) fn App() -> impl IntoView {
    let user_store = USER_MANAGER.initialize_user_store();
    provide_context(Store::new(user_store));

    view! {
        <Router>
            <UnauthenticatedRedirector />
            <Routes fallback=HomePage>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/auth") view=AuthPage />
            </Routes>
        </Router>
    }
}

#[component]
fn UnauthenticatedRedirector() -> impl IntoView {
    let user_store = expect_context::<Store<UserStore>>();

    Effect::new(move |_| {
        let logged_in = user_store.user().read().is_some();

        if !logged_in {
            let navigate = use_navigate();
            navigate("/auth", Default::default());
        }
    });

    ()
}
