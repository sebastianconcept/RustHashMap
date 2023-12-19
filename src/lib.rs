#![deny(unsafe_code)]
/* No `unsafe` needed! */

#[macro_use]
extern crate lazy_static;
extern crate benchmarking;
extern crate mut_static;

use ::safer_ffi::prelude::*;
use mut_static::MutStatic;
use rand::seq::SliceRandom;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::raw::c_uint;
use std::{
    collections::HashMap,
    ffi::CString,
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

/// `Storage` is a minimalistic key-value storage object keeping a clean API.
/// ```
/// let mut storage = Storage::new();
///
/// // Storing a value
/// storage.set("name".to_string(), "Alice".to_string());

/// // Reading the value
/// if let Some(name) = storage.get("name".to_string()) {
///     println!("Name: {}", name);
/// } else {
///     println!("Name not found.");
/// }
/// ```

#[derive(Clone, Default)]
pub struct Storage {
    pub store: HashMap<String, String>,
}

impl Storage {
    /// Retrieves the value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up in the storage.
    ///
    /// # Returns
    ///
    /// Returns `Some(value)` if the key exists, or `None` if not found.
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Checks if the storage includes a value at the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check.
    ///
    /// # Returns
    ///
    /// Returns `true` if the key exists, otherwise `false`.    
    pub fn includes(&self, key: String) -> bool {
        self.store.contains_key(&key)
    }

    /// Removes the value associated with the given key from the storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove.    
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }

    /// Resets the storage by clearing all key-value pairs.
    pub fn reset(&mut self) {
        self.store.clear()
    }

    /// Sets a new key-value pair in the storage.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set.
    /// * `value` - The value associated with the key.    
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Returns the number of key-value pairs in the storage.
    ///
    /// # Returns
    ///
    /// The size of the storage as an integer.    
    pub fn size(&self) -> i32 {
        self.store.len().try_into().unwrap()
    }
}

lazy_static! {
    /// Global static (protected) instance of the `Storage` struct.
    pub static ref STORAGE: MutStatic<Storage> = MutStatic::from(Storage::default());
    static ref KEYS: MutStatic<Vec<String>> = MutStatic::from(vec!());
}

pub static OUTPUT_FILE_NAME: &str = "output.txt";

/// Global static instance of a vector to store keys.
pub static SAMPLE_VALUE: &str = "{\"hlrSgsnNumber\":null,\"sponsoredImsi\":\"525053099536133\",\"vlrMscNumber\":\"792411112905\",\"mnc\":\"02\",\"vlrVlrNumber\":\"792411112905\",\"_id\":\"28981640290848413548099571056\",\"hlrMscNumber\":\"804107924111122\",\"#version\":-928585930571132360,\"hlrScfAddress\":\"14174000087\",\"customerImsi\":\"312300000591679\",\"sponsorName\":\"IMSI10\",\"sponsoredId\":\"10\",\"updatedTime\":\"2019-10-15T00:04:28.483+00:00\",\"hlrVlrNumber\":\"804107924111121\",\"maxGTLength\":15,\"rhToVLRGT\":\"6598541000\",\"vlrCalledTranslationType\":0,\"mme\":null,\"customerMsisdn\":\"879000000591679\",\"mcc\":\"250\",\"pilotMode\":0,\"skipCancelLocation\":null,\"packetSwitched\":false,\"sponsoredMsisdn\":\"65985001136133\",\"vlrSgsnNumber\":null,\"hlrHlrNumber\":\"14174000019\",\"mtSmsRewriteV1\":null,\"creationTime\":\"2019-10-15T00:04:28.483+00:00\",\"#instanceOf\":\"RHVlrImsiMapping\"}";

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
    writeln!(file, "{}", contents).unwrap();
    println!("{}", contents);
    file.flush().unwrap();
}

fn bench_inserts(quantity: u32, sample_payload: &str) {
    let bench_result = benchmarking::measure_function(move |measurer| {
        measurer.measure(|| {
            output(format!("Measuring Rust HashMap {} inserts...", quantity));
            for _i in 0..quantity {
                let id = Uuid::new_v4().to_string();
                let value = format!("{}-{}", id, sample_payload);
                KEYS.write()
                    .expect("Failed to grab a lock to mutate the KEYS object")
                    .push(id.clone());
                basic_set(id.to_owned(), value);
                // if i % 100000 == 0 {
                //     let content = format!("Adding {}: {}", i, id);
                //     output(content);
                // }
            }
        });
    })
    .unwrap();
    let contents = format!(
        "It took {:?} to perform {} insertions",
        bench_result.elapsed(),
        quantity
    );
    output(contents);
}

fn bench_reads(quantity: u32) {
    let bench_result = benchmarking::measure_function(move |measurer| {
        measurer.measure(|| {
            output(format!("Measuring Rust HashMap {} reads...", quantity));
            for _i in 0..quantity {
                let key = basic_keys_any();
                let _value = basic_get(key.clone()).unwrap();
                // if i % 100000 == 0 {
                //     let content = format!("Reading {} with {}", key, value);
                //     output(content);
                // }
            }
        });
    })
    .unwrap();
    let contents = format!(
        "It took {:?} to perform {} reads",
        bench_result.elapsed(),
        quantity
    );
    output(contents);
}

/// Run a basic benchmaking on writes and reads on the storage using an optional payload.
#[ffi_export]
pub fn benchmark(quantity: c_uint, sample_payload: Option<char_p::Ref<'_>>) {
    reset_output();
    output("Starting the benchmarking...".to_string());
    benchmarking::warm_up();
    output("Benchmarking warmed up and ready to go.".to_string());
    match sample_payload {
        Some(s) => bench_inserts(quantity, s.to_str()),
        None => bench_inserts(quantity, SAMPLE_VALUE),
    }
    bench_reads(quantity);
}

#[ffi_export]
pub fn keys_size() -> i32 {
    KEYS.read()
        .expect("Failed to grab a lock to read the KEYS object")
        .len()
        .try_into()
        .unwrap()
}

fn basic_keys_any() -> String {
    KEYS.read()
        .expect("Failed to grab a lock to read the KEYS object")
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
}

#[ffi_export]
pub fn keys_any() -> char_p::Box {
    let which = basic_keys_any();
    let value = CString::new(which.to_owned()).ok().unwrap();
    char_p::Box::try_from(value).unwrap()
}

#[ffi_export]
pub fn size() -> i32 {
    STORAGE
        .read()
        .expect("Failed to grab a lock to access the Storage object")
        .size()
}

#[ffi_export]
pub fn reset() {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .reset();
}

#[ffi_export]
pub fn includes(key: Option<char_p::Ref<'_>>) -> bool {
    match key {
        None => false,
        Some(k) => {
            let storage = STORAGE
                .read()
                .expect("Failed to grab a lock to read in the Storage object");
            storage.includes(k.to_string().clone())
        }
    }
}

#[ffi_export]
pub fn remove_key(key: char_p::Ref<'_>) {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .remove(key.to_string());
}

fn basic_set(key: String, value: String) {
    STORAGE
        .write()
        .expect("Failed to grab a lock to mutate the Storage object")
        .set(key, value);
}

#[ffi_export]
pub fn set(key: char_p::Ref<'_>, value: char_p::Ref<'_>) {
    basic_set(key.to_string(), value.to_string());
}

fn basic_get(key: String) -> Option<String> {
    STORAGE
        .read()
        .expect("Failed to grab a lock to read in the Storage object")
        .get(key)
}

#[ffi_export]
pub fn get(key: Option<char_p::Ref<'_>>) -> Option<char_p::Box> {
    match key {
        None => None,

        Some(k) => {
            let parsed_key = k.to_string();
            let value = basic_get(parsed_key);
            match value {
                None => None,
                Some(f) => {
                    let value = CString::new(f.clone()).ok().unwrap();
                    char_p::Box::try_from(value).ok()
                }
            }
        }
    }
}

#[ffi_export]
pub fn echo(key: char_p::Ref<'_>) -> char_p::Box {
    let answer = String::from(key.to_str());
    answer.try_into().unwrap()
}

#[ffi_export]
pub fn version() -> Option<char_p::Box> {
    let answer = String::from("0.1.2");
    Some(answer.try_into().unwrap())
}

#[ffi_export]
fn concat(first: char_p::Ref<'_>, second: char_p::Ref<'_>) -> char_p::Box {
    let str1 = first.to_str(); // : &'_ str
    let str2 = second.to_str(); // : &'_ str
    format!("{}{}", str1, str2) // -------+
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
