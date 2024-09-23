# Requests Lifecycle

## Overview

The lifecycle of requests when running the server as a Wasm Component
is kind of special:

Instead of running your own HTTP server, you offload some responsibilities
to the host runtime so you can focus on the "business logic" of your
application.

For example, if you run the component in the wasmCloud eco-system, you
offload auto-scaling (since components are initiated on a
per-request basis) and distribution of your app! :rocket:
(Along other fun stuff like transport layer and protocol goodies).

In the context of leptos, we still have to do the *Routing*, i.e.
what "business logic" to call based on the HTTP `method` and `path` of the
request.

## Steps

1. The user request leads to the invokation of the `wasi:http/incoming-handler`
   exported function.
2. The user manually register its server functions. Since "life-before-main"
   is not supported by the Rust compiler when targeting `wasm32-*` targets,
   we cannot use the `inventory` crate [see this thread][global-res].
3. **Fast Path**: We start with server function routing since it's cheaper
   than going through the rendering logic. If we have a match, handle the
   server function (with [context][use-context], but I need more background
   on that because I am not sure how it is implemented in 0.7, I feel like
   the `runtime` has been replaced with the `Owner` struct).
4. **Extract Routes**: We *blank shoot* render the user `App` component
   to extract routes and translate them in a structure that allows us
   to map the request `path` and `method` with a specific handler that
   will render and stream the *dehydrated* SSR page.
5. **SSR**: See (4), we will implement all SSR Modes (for fun).

:tada:

[global-res]: https://internals.rust-lang.org/t/global-registration-a-kind-of-pre-rfc/20813
[use-context]: https://docs.rs/leptos/latest/leptos/fn.use_context.html
