use futures::future;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let increment_star = ServerAction::<UpdateCount>::new();

    let count = Resource::new(move || increment_star.version().get(), |_| get_count());

    view! {
        <div class="flex w-screen h-screen justify-center items-center">
            <div class="flex flex-col w-2/3 p-5 py-12 bg-gray-800 text-gray-100 items-center gap-5">
                <picture class="img w-32">
                    <source srcset="https://raw.githubusercontent.com/wasmCloud/wasmCloud/dc847d96740541a7d57cd5cee074d29c1122f9bf/brand/icon/green/wasmCloud.icon-green.svg" media="(prefers-color-scheme: dark)" />
                    <img src="https://raw.githubusercontent.com/wasmCloud/wasmCloud/dc847d96740541a7d57cd5cee074d29c1122f9bf/brand/icon/green/wasmCloud.icon-green.svg" alt="wasmCloud Logo" height="200" width="400" />
                </picture>

                <h1 class="text-3xl md:text-xl">"Welcome to Leptos hosted on WebAssembly WASI Component!"</h1>

                <h2 class="text-xl md:text-lg">
                <Suspense fallback=|| view! { "Loading star count..." }>
                    {move || count.get()} stars given!
                </Suspense>
                </h2>

                <ActionForm action=increment_star>
                    <button>
                        Star me!
                    </button>
                </ActionForm>
            </div>
        </div>
    }
}

#[server]
pub async fn update_count() -> Result<(), ServerFnError> {
    use crate::bindings::wasi::filesystem::{preopens::get_directories, types::{DescriptorFlags, OpenFlags, PathFlags}};

    println!("User requested an update to the store");

    let updated_value = get_count().await? + 1;
    let directories = get_directories();
    let (fd, _) = directories.first().expect("no directory given");

    fd
        .open_at(PathFlags::empty(), "store.txt", OpenFlags::CREATE, DescriptorFlags::WRITE)
        .map(|fd| {
            let stream = fd.write_via_stream(0).expect("failed to open a stream to write");
            stream.blocking_write_and_flush(updated_value.to_string().as_bytes()).expect("could not write to the store");
            ()
        })
        .map_err(|err| {
            ServerFnError::ServerError(err.message().to_string())
        })
}

#[server]
pub async fn get_count() -> Result<u64, ServerFnError> {
    use crate::bindings::wasi::filesystem::{preopens::get_directories, types::{DescriptorFlags, OpenFlags, PathFlags}};

    println!("Getting the store");
    let directories = get_directories();
    let (fd, _) = directories.first().expect("no directory given");

    match fd.open_at(PathFlags::empty(), "store.txt", OpenFlags::CREATE, DescriptorFlags::READ) {
        Err(err) => {
            println!("could not open store for reading");
            println!("reason: {}", err.message());
            Ok(0)
        },
        Ok(fd) => {
            let file_size = fd.stat().expect("should be able to stat").size;
            match fd.read_via_stream(0) {
                Err(err) => {
                    println!("could not open stream to store");
                    println!("reason: {}", err.message());
                    Ok(0)
                },
                Ok(stream) => {
                    let mut store: Vec<u8> = Vec::new();
                    loop {
                        if store.len() as u64 >= file_size {
                            break;
                        }

                        match stream.blocking_read(256) {
                            Err(_) => return Ok(0),
                            Ok(data) => {
                                store.extend(data);
                            }
                        }
                    }
                    let result = String::from_utf8(store).expect("no utf-8");
                    Ok(result.parse::<u64>().unwrap_or(0))
                }
            }
        }
    }
}