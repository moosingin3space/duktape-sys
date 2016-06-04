// Builds the duktape C library
extern crate gcc;
extern crate bindgen;

fn main() {
    gcc::Config::new()
        .file("c_src/duktape.c")
        .include("c_src")
        .compile("libduktape.a");
}
