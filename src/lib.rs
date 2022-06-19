#![deny(unsafe_code)]
/* No `unsafe` needed! */

#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use ::safer_ffi::prelude::*;
use libc::c_char;
use mut_static::MutStatic;
use safer_ffi::c;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::ops::DerefMut;
use std::ptr;
use std::str;
use std::sync::Mutex;

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
    println!("set:  {} -> {}", key.to_str(), value.to_str());
    // STORAGE.lock().unwrap().insert("cds", "ffff");
    // let k = CString::new(key.to_str()).expect("CString::new failed to create key");
    // let v= CString::new(value.to_str()).expect("CString::new failed to create value");

    // let k = CString::new(key.to_str())
    //     .expect("CString::new failed to create key")
    //     .into_string()
    //     .unwrap();
    // let v = CString::new(value.to_str())
    //     .expect("CString::new failed to create value")
    //     .into_string()
    //     .unwrap();

    let k = key.to_string();
    let v = value.to_string();
    STORAGE.write().unwrap().set(&k, &v);
}

#[ffi_export]
pub fn get(key: char_p::Box) -> char_p::Box {
    // let answer = STORAGE
    //     .lock()
    //     .unwrap()
    //     .get(key.to_str())
    //     .unwrap()
    //     .to_string();
    // let answer = STORAGE.lock().unwrap().get(key.to_str()).unwrap_or(ptr::null());
    //    let answer = STORAGE.read().unwrap().get(&key.to_string()).unwrap().to_owned();
    //    answer.try_into().unwrap()
    STORAGE
        .read()
        .unwrap()
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
pub fn echo(key: char_p::Box) -> char_p::Box {
    let answer = String::from(key.to_str());
    answer.try_into().unwrap()
}

static HELLO: &'static str = "hello from the rust lib";

// alternative, if a string literal:

#[ffi_export]
pub fn getCstr() -> char_p::Ref<'static> {
    c!("Hello, World!!!!!!!!!!!")
}

#[ffi_export]
pub fn setCstr(key: char_p::Box, value: char_p::Box) {
    println!("setCstr:  {} -> {}", key, value);
}

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

// fn main () {
//     STORAGE.insert(&"fdf",&"something");
// }
