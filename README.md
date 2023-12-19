# RustHashMap
FFI reachable C-Shared lib made with a thread-safe HashMap in Rust.

## Build
To create a fresh version, from the project root directory, run:

    ./build.sh

That will create a `target/debug/librusthashmap.dynlib` in macOS, also its associated headers file.

## Inner benchmark
From [Pharo](https://pharo.org/) running the built-in Rust benchmark on 10K operations:

```Smalltalk
sample1 := '{"id":123,"name":"Sample JSON","description":"This is a sample JSON object with approximately 1024 bytes of data. It''s used for demonstration purposes.","tags":["json","sample","data"],"details":{"created_at":"2023-04-01T12:00:00","updated_at":"2023-04-01T14:30:00","status":"active"},"values":[1,2,3,4,5,6,7,8,9,10],"settings":{"enabled":true,"threshold":50,"options":["option1","option2","option3"]},"comments":[{"user":"user1","text":"This is a comment."},{"user":"user2","text":"Another comment here."}]}'.

RustHashMap benchmark: 10 * 1000 withPayload: sample1.
```
On this hardware [1] it renders:

```
➜  PharoRustHashMap ./pharo-ui compare.image
Starting the benchmarking...
Benchmarking warmed up and ready to go.
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
It took 13.681405ms to perform 10000 insertions
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
It took 8.764975ms to perform 10000 reads
```

## FFI usage from Pharo compared to Redis

With [RediStick](https://github.com/mumez/RediStick) and [ABBench](https://github.com/emdonahue/ABBench) installed you can benchmark this lib against Redis. Here is a snippet with the numbers provided with this hardware [1]:

```Smalltalk
RsRedisConnectionPool primaryUrl: 'sync://localhost:6379'.
redis := RsRedisProxy of: #client1.

sample1 := '{"id":123,"name":"Sample JSON","description":"This is a sample JSON object with approximately 1024 bytes of data. It''s used for demonstration purposes.","tags":["json","sample","data"],"details":{"created_at":"2023-04-01T12:00:00","updated_at":"2023-04-01T14:30:00","status":"active"},"values":[1,2,3,4,5,6,7,8,9,10],"settings":{"enabled":true,"threshold":50,"options":["option1","option2","option3"]},"comments":[{"user":"user1","text":"This is a comment."},{"user":"user2","text":"Another comment here."}]}'.

keys := (1 to: 10000) collect:[ :e | UUID new asString36 ].
values := (1 to: 10000) collect:[ :e | UUID new asString36, '-',sample1 ].
source := Dictionary newFromAssociations: (keys withIndexCollect:[ :k :i | k -> (values at: i)]).

"Setting values in RustHashMap lib via FFI"
Time millisecondsToRun: [ source keysAndValuesDo: [ :k :v | RustHashMap set: k with: v ] ]. 
"293"

"Getting values from RustHashMap lib via FFI"
Time millisecondsToRun: [ keys collect: [ :k | RustHashMap get: k ] ]. 
"656"

"Setting values in a local Redis"
Time millisecondsToRun: [ source keysAndValuesDo: [ :k :v | redis at: k put: v ] ]. 
"1582"

"Getting values from a local Redis"
Time millisecondsToRun: [ keys collect: [ :k | redis at: k ] ]. 
"1501"

"Comparing repeated same write"
ABBench bench: [ ABBench 
	a: [ RustHashMap set: keys anyOne with: values anyOne ] 
	b: [ redis at: keys anyOne put: values anyOne ] ]. 
"B is 83.71% SLOWER than A"

"Comparing repeated same read"
ABBench bench: [ ABBench 
	a: [ RustHashMap get: keys anyOne ] 
	b: [ redis at: keys anyOne  ] ]. 
 "B is 62.46% SLOWER than A"
```

## Example of usage in Rust:

```rust
// Import the Storage struct and associated lazy_static instances.
use crate::Storage;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref STORAGE: Mutex<Storage> = Mutex::new(Storage::new());
}

fn main() {
    // Create a mutable reference to the global STORAGE instance.
    let mut storage = Storage::new();

    // Set key-value pairs.
    storage.set("name".to_string(), "John".to_string());
    storage.set("age".to_string(), "30".to_string());

    // Retrieve and print values.
     if let Some(name) = storage.get("name".to_string()) {
         println!("Name: {}", name);
     }

     if let Some(age) = storage.get("age".to_string()) {
         println!("Age: {}", age);
     }
}
```

In this example, we create a `Storage` instance, set key-value pairs, and retrieve values.
The `lazy_static` crate is used for creating a global, lazy-initialized instance of `Storage`.

## Dependencies
This doesn't have any dependencies other than `Rust` itself:

    https://www.rust-lang.org/tools/install


[1] An Intel based MacBook Pro, 2,5 GHz Quad-Core Intel Core i7