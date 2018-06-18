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
        "https://archive.org/download/78_little-brown-jug_glenn-miller-and-his-orchestra-glenn-miller_gbia0015205a/Little%20Brown%20Jug%20-%20Glenn%20Miller%20and%20his%20Orchestra.mp3";

    let mut testdata_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testdata_folder.push("data");

    let mut testdata_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testdata_file.push("data/test.mp3");

    let command = format!(
        "mkdir -p {}; wget {} -O {}",
        testdata_folder.to_str().unwrap(),
        testdata_url,
        testdata_file.to_str().unwrap()
    );
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}
