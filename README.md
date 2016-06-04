# Low-level, unsafe Rust bindings to the Duktape JavaScript engine

[Duktape](http://duktape.org) is an embeddable JavaScript engine with a focus on
portability and compact footprint. This repository hosts 
[bindgen](https://github.com/crabtw/rust-bindgen) generated Rust bindings to the
Duktape JavaScript engine's C library.

For documentation, consult the 
[Duktape C API documentation](http://duktape.org/api.html), as this is nothing
more than a set of automatically-generated bindings to it.

## Usage

To use `duktape-sys`, first add this to your `Cargo.toml`:

```toml
[dependencies]
duktape-sys = "0.1"
```

Then, add this to your crate root:
```rust
extern crate duktape_sys;
```
