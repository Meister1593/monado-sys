#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Re-export the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));