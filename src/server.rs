use std::task::Poll;

use bytes::Bytes;
use futures::stream;
use leptos::{config::get_configuration, error::Error, task::Executor};
use leptos_wasi::prelude::{Body, WasiExecutor};
use wasi::{exports::http::incoming_handler::Guest, filesystem::{preopens::get_directories, types::{DescriptorFlags, OpenFlags, PathFlags}}, http::{proxy::export, types::{IncomingRequest, ResponseOutparam}}};

use crate::{pages::home::{GetCount, UpdateCount}, routes::{shell, App}};

struct LeptosServer;

// NB(raskyld): for now, the types to use for the HTTP handlers are the one from
// the `leptos_wasi` crate, not the one generated in your crate.
impl Guest for LeptosServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        // Initiate a single-threaded [`Future`] Executor so we can run the
        // rendering system and take advantage of bodies streaming.
        let executor = WasiExecutor::new(leptos_wasi::executor::Mode::Stalled);
        Executor::init_local_custom_executor(executor.clone()).expect("cannot init future executor");
        executor.run_until(async {
            handle_request(request, response_out).await;
        })
    }
}

async fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
    use leptos_wasi::prelude::Handler;
    
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;

    Handler::build(request, response_out)
        .expect("could not create handler")
        .static_files_handler("/pkg", serve_static_files)
        .with_server_fn::<UpdateCount>()
        .with_server_fn::<GetCount>()
        // Fetch all available routes from your App.
        .generate_routes(App)
        // Actually process the request and write the response.
        .handle_with_context( move || shell(leptos_options.clone()), || {}).await.expect("could not handle the request");
}

fn serve_static_files(path: String)
    -> Option<Body>
{
    println!("serving {}", &path);
    let directories = get_directories();
    let path = path.strip_prefix("/").unwrap_or(&path);
    let (fd, _) = directories.first().expect("there seems to be no static files to serve");

    match fd.open_at(PathFlags::empty(), &path, OpenFlags::empty(), DescriptorFlags::READ) {
        Err(err) => {
            println!("could not serve file {}", path);
            println!("reason: {}", err.message());
            return None;
        },
        Ok(fd) => {
            let file_size = fd.stat().expect("should be able to stat").size;
            match fd.read_via_stream(0) {
                Err(err) => {
                    println!("could not open stream to file {}", path);
                    println!("reason: {}", err.message());
                    return None;
                },
                Ok(stream) => {
                    let mut read_bytes: u64 = 0;
                    return Some(
                        Body::Async(
                            Box::pin(stream::poll_fn(move |_| -> Poll<Option<Result<Bytes, Error>>> {
                                if read_bytes >= file_size {
                                    return Poll::Ready(None)
                                }

                                match stream.blocking_read(256) {
                                    Err(err) => Poll::Ready(Some(Err(err.into()))),
                                    Ok(data) => {
                                        read_bytes += data.len() as u64;
                                        return Poll::Ready(Some(Ok(Bytes::from(data))));
                                    }
                                }
                            }))
                        )
                    );
                }
            }
        }
    }
}

export!(LeptosServer with_types_in wasi);
