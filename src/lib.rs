#![deny(unsafe_code)]
/* No `unsafe` needed! */

#[macro_use]
extern crate lazy_static;
extern crate benchmarking;
extern crate mut_static;

use ::safer_ffi::prelude::*;
use core::option::Option;
use mut_static::MutStatic;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;
use std::{
    collections::HashMap,
    ffi::CString,
    fs::{self, File},
};

pub struct Storage {
    pub store: HashMap<String, String>,
}

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
        self.store.insert(key.to_string(), value.to_string());
    }
    pub fn remove(&mut self, key: &String) {
        self.store.remove(key);
    }
    pub fn includes(&self, key: &String) -> bool {
        self.store.contains_key(key)
    }
    pub fn reset(&mut self) {
        self.store.clear()
    }
    pub fn size(&self) -> i32 {
        self.store.len().try_into().unwrap()
    }
}

lazy_static! {
    pub static ref STORAGE: MutStatic<Storage> = MutStatic::from(Storage::new());
}

pub static OUTPUT_FILE_NAME: &str = "output.txt";

fn reset_output() {
    if Path::new(OUTPUT_FILE_NAME).exists() {
        fs::remove_file(OUTPUT_FILE_NAME).unwrap();
    }
    File::create(OUTPUT_FILE_NAME).unwrap();
}

fn output(contents: String) {
    let is_new = Path::new(OUTPUT_FILE_NAME).exists();
    let mut file = OpenOptions::new()
        .create(!is_new)
        .write(true)
        .append(true)
        .open(OUTPUT_FILE_NAME)
        .unwrap();
    write!(file, "{}\n", contents).unwrap();
    println!("{}", contents);
    file.flush().unwrap();
}

#[ffi_export]
pub fn benchmark(quantity: u8) {
    reset_output();
    output("Starting the benchmarking...".to_string());
    // benchmarking::warm_up();
    // print!("Starting benchmarking of {}", quantity);
    // let mut storage = STORAGE
    //     .write()
    //     .expect("Failed to grab a lock to mutate the Storage object");
    // storage.reset();
}

#[ffi_export]
pub fn size() -> i32 {
    let storage = STORAGE
        .read()
        .expect("Failed to grab a lock to access the Storage object");
    storage.size()
}

#[ffi_export]
pub fn reset() {
    let mut storage = STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object");
    storage.reset();
}

#[ffi_export]
pub fn includes(key: Option<char_p::Box>) -> bool {
    let answer = match key {
        None => false,
        Some(k) => {
            let storage = STORAGE
                .read()
                .expect("Failed to grab a lock to read in the Storage object");
            storage.includes(&k.to_string())
        }
    };
    answer
}

#[ffi_export]
pub fn remove(key: char_p::Box) {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .remove(&key.to_string());
}

#[ffi_export]
pub fn set(key: char_p::Box, value: char_p::Box) {
    let k = key.to_string();
    let v = value.to_string();
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .set(&k.to_owned(), &v.to_owned());
}

#[ffi_export]
pub fn get(key: Option<char_p::Box>) -> Option<char_p::Box> {
    let answer = match key {
        None => None,
        Some(k) => {
            let storage = STORAGE
                .read()
                .expect("Failed to grab a lock to read in the Storage object");
            let value = storage.get(&k.to_string());
            match value {
                None => None,
                Some(r) => {
                    let value = CString::new(r.to_owned()).ok().unwrap();
                    char_p::Box::try_from(value).ok()
                }
            }
        }
    };
    answer
}

#[ffi_export]
pub fn echo(key: char_p::Box) -> char_p::Box {
    let answer = String::from(key.to_str());
    answer.try_into().unwrap()
}

#[ffi_export]
pub fn version() -> char_p::Box {
    let answer = String::from("0.1.1");
    answer.try_into().unwrap()
}

#[ffi_export]
fn concat(fst: char_p::Ref<'_>, snd: char_p::Ref<'_>) -> char_p::Box {
    let fst = fst.to_str(); // : &'_ str
    let snd = snd.to_str(); // : &'_ str
    format!("{}{}", fst, snd) // -------+
        .try_into() //                   |
        .unwrap() // <- no inner nulls --+
}

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("target/debug/librusthashmap.h")?
        .generate()
}
