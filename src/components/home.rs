use leptos::prelude::*;

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello, world!"</h1>
            <p>"This is a simple Leptos app."</p>
        </div>
    }
}