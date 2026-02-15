use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod content;
mod pages;

use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:class="dark" attr:data-theme="dark" />

        <Title text="Piyush | Portfolio" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
