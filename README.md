# leptos-wasmcloud

> This is just a meta-repo for tracking my progress trying to make
> Leptos output the server-side binary as a WebAssembly Component
> instead.

## Demo

https://www.youtube.com/watch?v=SAAyukYJGeY&t=141s

## Overview

I started this repository focusing on the specific integration of [Leptos][leptos]
with [wasmcloud][wasmcloud], but I ended up spliting my work in two phases:

1. :sparkles: **Standard WASI**: Make Leptos output a standard component
   that is built solely on standards.
2. :rocket: **`wasmcloud`-specifics**: Speed up the deployment of the component
   on [**wasmcloud Lattices**][lattice].

You can find details of the initial study I made [here](docs/initial-study.md).

Step (1) is done and has its own repository hosted on
[`leptos-rs/leptos_wasi`](https://github.com/leptos-rs/leptos_wasi) :tada:

[wasm-component]: https://component-model.bytecodealliance.org/design/components.html
[leptos]: https://www.leptos.dev
[wasmcloud]: https://wasmcloud.com
[lattice]: https://wasmcloud.com/docs/concepts/lattice

## Progress

* [x] Start thinking about the overall architecture.
* [x] Get in touch with Leptos maintainers on their Discord to get tips.
* [x] Decide whether it should be in-tree Leptos code or a dedicated repo.
* [x] Setup a PoC Rust project that works with `cargo leptos` and `cargo component`
* [x] Fixes #1 (On my fork, it's fixed, PR to upstream is open)
* [x] Document a bit the request lifecycle
* [x] Provide utils in the form of a `leptos/integrations/` crate
   * [x] Server Functions Registry
   * [x] Router (Server fn, fallback to render logic)
   * [x] Abstract low-level Req/Res of the WASI interface
   * [x] Integration with Leptos Contexts
* [x] Propose my contribution upstream or extract the utils to a standalone crate
  (The PR is [there](https://github.com/leptos-rs/leptos/pull/3063))
* [ ] Implement an example/template app
* [ ] Ideas of integration with wasmCloud tools
* [x] Share with the wasmCloud community
