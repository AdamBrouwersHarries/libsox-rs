
// Wrap the "raw" ffi interface in a private library, so that implementers, 
// but not users can access it. 

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));