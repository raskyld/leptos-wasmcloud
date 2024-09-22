#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[allow(warnings)]
mod bindings;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        /// This is the entrypoint called by the JS "igniter" script.
        #[wasm_bindgen]
        pub fn hydrate() {
            use routes::*;

            console_error_panic_hook::set_once();

            leptos::mount_to_body(AppRouter);
        }
    }
}
