use leptos::prelude::*;

#[component]
pub(crate) fn AuthPage() -> impl IntoView {
    view! {
        <div>
            <h1>"Authentication Page"</h1>
            <p>"This is the authentication page."</p>
            <LoginButton />
        </div>
    }
}

#[component]
fn LoginButton() -> impl IntoView {
    view! {
        <button>"Log in with GitHub"</button>
    }
}