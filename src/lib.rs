#![deny(unsafe_code)] use safer_ffi::c;
/* No `unsafe` needed! */
use ::safer_ffi::prelude::*;
use std::collections::HashMap;
use libc::c_char;
use std::ffi::{CStr, CString, NulError};
use std::str;

// #[ffi_export]
// pub fn freeStorage(s: HashMap<K, V>::Box) {
//     println!("freeStorage");
//     drop(s.unwrap());
// }

// #[ffi_export]
// pub fn newStorage() -> HashMap<K, V>::Box {
//     // char_p::new("Hello, World!\0")
//     Box::new(HashMap::new())
// }

#[ffi_export]
pub fn getOwnedCStr() -> char_p::Box {
    char_p::new("Hello, World!\0")
}

#[ffi_export]
pub fn freeOwnedCStr(p: Option<char_p::Box>) {
    drop(p);
}

// #[ffi_export]
// pub fn get(key: char_p::Box) -> char_p::Ref<'static> {
//     let value = key.to_str().to_owned();
    // let c_buf: *const c_char = unsafe { key.to_str() };
    // let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    // let str_slice: &str = c_str.to_str().unwrap();
    // let str_buf: String = str_slice.to_owned();
    // value
    // CString::new("Hello World").unwrap()
// }

// #[ffi_export]
// pub fn pget(key: char_p::Box) -> c_char {
//     let mut contacts = HashMap::new();
//     contacts.insert("Daniel", "798-1364");
//     contacts.insert("Ashley", "645-7689");
//     contacts.insert("Katie", "435-8291");
//     contacts.insert("Robert", "956-1745");
//     let answer =contacts.get("Daniel").unwrap();
//     let cstring= CString::new(answer.to_owned()).expect("CString::new failed");
//     cstring.as_ptr()
// }

#[ffi_export]
pub fn dummyGet(_key: char_p::Box) -> *const c_char {
    let answer = String::from("Dummy answer\0");
    let cstring = CString::new(answer).expect("CString::new failed");
    cstring.as_ptr()
}

static HELLO: &'static str = "hello from rust";

#[ffi_export]
pub fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}


#[ffi_export]
pub fn set(key: char_p::Box, value: char_p::Box ) {
    println!("setCstr:  {} -> {}", key, value);
}

// alternative, if a string literal:

#[ffi_export]
pub fn getCstr() -> char_p::Ref<'static> {
    c!("Hello, World!!!!!!!!!!!")
}

#[ffi_export]
pub fn setCstr(key: char_p::Box, value: char_p::Box ) {
    println!("setCstr:  {} -> {}", key, value);
}


#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers () -> ::std::io::Result<()>
{
    ::safer_ffi::headers::builder()
        .to_file("target/debug/librusthashmap.h")?
        .generate()
}

#[ffi_export]
fn concat (fst: char_p::Ref<'_>, snd: char_p::Ref<'_>)
  -> char_p::Box
{
   let fst = fst.to_str(); // : &'_ str
   let snd = snd.to_str(); // : &'_ str
   format!("{}{}", fst, snd) // -------+
      .try_into() //                   |
      .unwrap() // <- no inner nulls --+
}