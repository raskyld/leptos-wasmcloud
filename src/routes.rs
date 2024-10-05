use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Router, Routes, Route},
    path
};

use crate::pages::home::Home;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
                <Stylesheet id="leptos" href="/pkg/leptos_wasmcloud.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="A website running its server-side as a WASI Component :D"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="white"/>

        <Title text="Welcome to Leptos X WebAssembly!"/>

        <Router>
            <main>
                <Routes fallback>
                    <Route path=path!("/home") view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
