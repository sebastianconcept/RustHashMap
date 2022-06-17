#![deny(unsafe_code)] /* No `unsafe` needed! */
use ::safer_ffi::prelude::*;

#[ffi_export]
pub fn getOwnedCStr() -> char_p::Box {
    char_p::new("Hello, World!\0")
}

#[ffi_export]
pub fn freeOwnedCStr(p: Option<char_p::Box>) {
    drop(p);
}

// alternative, if a string literal:

#[ffi_export]
pub fn getCstr() -> char_p::Ref<'static> {
    c!("Hello, World!!!!!!!!!!!")
}

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers () -> ::std::io::Result<()>
{
    ::safer_ffi::headers::builder()
        .to_file("target/debug/librusthashmap.h")?
        .generate()
}
