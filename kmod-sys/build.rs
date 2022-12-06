extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn genbindings(path: &str) -> Result<bindgen::Bindings, bindgen::BindgenError> {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(path)
        // Finish the builder and generate the bindings.
        .generate()
}

fn try_genbindings() -> Option<bindgen::Bindings> {
    for header in &["wrapper.h", "fallback.h"] {
        if let Ok(bindings) = genbindings(header) {
            return Some(bindings);
        }
    }

    None
}

fn main() {
    // Tell cargo to tell rustc to link the system kmod
    // shared library.
    println!("cargo:rustc-link-lib=kmod");

    let bindings = try_genbindings()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
