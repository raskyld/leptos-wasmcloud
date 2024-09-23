# leptos-wasmcloud

> This is just a meta-repo for tracking my progress trying to make
> Leptos output the server-side binary as a WebAssembly Component
> instead.

## Overview

The goal is to make [Leptos][leptos] output a
[WebAssembly Component][wasm-component] that will serve the HTTP requests
enabling the different
[SSR Modes of Leptos](https://book.leptos.dev/ssr/23_ssr_modes.html).

I started this repository focusing on the specific integration of [Leptos][leptos]
with [wasmcloud][wasmcloud], but I ended up spliting my work in two phases:

1. :sparkles: **Standard WASI**: Make Leptos output a standard component
   that is built solely on standards.
2. :rocket: **`wasmcloud`-specifics**: Speed up the deployment of the component
   on [**wasmcloud Lattices**][lattice].

You can find details of the initial study I made [here](docs/initial-study.md).

[wasm-component]: https://component-model.bytecodealliance.org/design/components.html
[leptos]: https://www.leptos.dev
[wasmcloud]: https://wasmcloud.com
[lattice]: https://wasmcloud.com/docs/concepts/lattice

## Progress

* [x] Start thinking about the overall architecture.
* [x] Get in touch with Leptos maintainers on their Discord to get tips.
* [ ] Decide whether it should be in-tree Leptos code or a dedicated repo.
* [x] Setup a PoC Rust project that works with `cargo leptos` and `cargo component`
* [x] Fixes #1 (On my fork, it's fixed, PR to upstream is open)
* [x] Document a bit the request lifecycle
* [ ] Provide utils in the form of a `leptos/integrations/` crate
   * [ ] Server Functions Registry
   * [ ] Router (Server fn, fallback to render logic)
   * [ ] Abstract low-level Req/Res of the WASI interface
   * [ ] Integration with Leptos Contexts
* [ ] Propose my contribution upstream or extract the utils to a standalone crate
* [ ] Implement an example/template app
* [ ] Ideas of integration with wasmCloud tools
* [ ] Share with the wasmCloud community
