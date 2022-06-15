#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
    non_upper_case_globals, unused_assignments, unused_mut)]
use std::ffi::{CStr, CString};
use libc;
use std::os::raw::c_char;

#[repr(C)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[repr(u32)]
pub enum Foo {
    A = 1,
    B,
    C,
}

#[no_mangle]
pub unsafe extern "C" fn getVersion() -> *const c_char {
    // let version = "lib version 0.1.0".to_owned();
    // match version {
    //     Ok(v) => CString::new(v.to_owned()),
    //     Err(e) => (CString::new("Error trying to return the lib version")),
    // }
    let version = "lib version 0.1.0\0";
    version.as_ptr().cast::<c_char>()
}


extern "C" fn returns_string() -> *const c_char {
  "hi, there, I'm gonna be a C string\0".as_ptr().cast::<c_char>() //adding the null terminator is not optional. Gotta do it by hand.
}

#[no_mangle]
pub unsafe extern "C" fn print() {
    println!("Hello from Rust via FFI")
}

#[no_mangle]
pub unsafe extern "C" fn get_origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

#[no_mangle]
pub unsafe extern "C" fn print_foo(foo: *const Foo) {
    println!(
        "{}",
        match *foo {
            Foo::A => "a",
            Foo::B => "b",
            Foo::C => "c",
        }
    );
}
