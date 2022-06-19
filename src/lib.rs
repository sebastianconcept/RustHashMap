#![deny(unsafe_code)]
/* No `unsafe` needed! */

#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use ::safer_ffi::prelude::*;
use core::option::Option;
use libc::c_char;
use mut_static::MutStatic;
use std::collections::HashMap;
use std::ffi::CString;

pub struct Storage {
    pub store: HashMap<String, String>,
}
// pub store: Mutex<HashMap<String, String>>

impl Storage {
    pub fn new() -> Self {
        Storage {
            store: HashMap::new(),
        }
    }
    pub fn get(&self, key: &String) -> Option<&String> {
        self.store.get(key)
    }
    pub fn set(&mut self, key: &String, value: &String) {
        self.store.insert(key.to_owned(), value.to_owned());
    }
}

// // static ref STORAGE: Mutex<HashMap<str, str>> = Mutex::new(HashMap::new());

lazy_static! {
    pub static ref STORAGE: MutStatic<Storage> = MutStatic::from(Storage::new());
    // pub static ref STORAGE: Storage = Storage::new();
}

#[ffi_export]
pub fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}

#[ffi_export]
pub fn set(key: char_p::Box, value: char_p::Box) {
    let k = key.to_string();
    let v = value.to_string();
    STORAGE.write().unwrap().set(&k, &v);
}

#[ffi_export]
pub fn get(key: char_p::Box) -> char_p::Box {
    STORAGE
        .read()
        .expect("Failed to grab a lock to read in the Storage object")
        .get(&key.to_string())
        .unwrap()
        .to_owned()
        .try_into()
        .unwrap()
}

#[ffi_export]
pub fn getOwnedCStr() -> char_p::Box {
    char_p::new("Hello, World!\0")
}

#[ffi_export]
pub fn freeOwnedCStr(p: Option<char_p::Box>) {
    drop(p);
}

#[ffi_export]
pub fn echo(key: char_p::Box) -> char_p::Box {
    let answer = String::from(key.to_str());
    answer.try_into().unwrap()
}

static HELLO: &'static str = "hello from the rust lib";

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("target/debug/librusthashmap.h")?
        .generate()
}

#[ffi_export]
fn concat(fst: char_p::Ref<'_>, snd: char_p::Ref<'_>) -> char_p::Box {
    let fst = fst.to_str(); // : &'_ str
    let snd = snd.to_str(); // : &'_ str
    format!("{}{}", fst, snd) // -------+
        .try_into() //                   |
        .unwrap() // <- no inner nulls --+
}
