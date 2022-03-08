extern crate cossack;
use cossack::DB;

fn main() {
    
    let mut db = DB::open();

    db.put(b"key", b"this is value");

    let value = db.get(b"key");
    println!("value: {:?}", value);

    let _result = db.delete(b"key");

    db.close();
}