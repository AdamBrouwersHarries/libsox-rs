# rust-libsox

Rust bindings to the libsox audio manipulation library. 

Current status: 

    - bindings with bindgen
    - preliminary semantic tests (e.g. opening/closing)
    - preliminary higher level constructs

TODO:

    - usable high level rust interface

# Usage

Prerequisites: 

    - libsox/sox installed, and in a place where the C linker and compiler can find it
    - `wget` and `sh` for testing purposes.

Compilation: 

`cargo build; cargo test`

