extern crate cossack;
use cossack::{put, get, delete};

fn main() {
    print!("Hello world.");

    put(b"key", b"this is value");
    let value = get(b"key");
    println!("value: {:?}", value);

    let _result = delete(b"key");
}