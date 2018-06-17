extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // tell cargo to tell rust to link the system libsox shared library
    println!("cargo:rustc-link-lib=sox");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Download some data for our tests.
    let testdata_url =
        "https://freemusicarchive.org/music/download/3deb8f20d0eb5ce089d849141d3b8e07f7839cb3";
    let testdata_file = "data/test.mp3";
    let command = format!("mkdir -p data; wget {} -O {}", testdata_url, testdata_file);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}
