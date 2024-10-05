use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    let increment = move |_| {
        set_value.update(|v| *v += 1);
    };

    view! {
        <div class="flex w-screen h-screen justify-center items-center">
            <div class="flex flex-col w-2/3 p-5 py-12 bg-gray-800 text-gray-100 items-center gap-5">
                <picture class="img w-32">
                    <source srcset="https://raw.githubusercontent.com/wasmCloud/wasmCloud/dc847d96740541a7d57cd5cee074d29c1122f9bf/brand/icon/green/wasmCloud.icon-green.svg" media="(prefers-color-scheme: dark)" />
                    <img src="https://raw.githubusercontent.com/wasmCloud/wasmCloud/dc847d96740541a7d57cd5cee074d29c1122f9bf/brand/icon/green/wasmCloud.icon-green.svg" alt="wasmCloud Logo" height="200" width="400" />
                </picture>

                <h1 class="text-3xl md:text-xl">"Welcome to Leptos hosted on WebAssembly WASI Component!"</h1>

                <h2 class="text-xl md:text-lg">{value} stars given!</h2>

                <button on:click=increment>
                    Star me!
                </button>
            </div>
        </div>
    }
}