/// re-impl leveldb in rust
/// 
/// 

mod db_impl;

pub fn put(key: &[u8], value: &[u8]) {
    println!("key : {:?}", key.as_ref());
    println!("value : {:?}", value.as_ref());
}

pub fn get(key: &[u8]) -> Option<Vec<u8>> {
    db_impl::_get(key)
}

pub fn delete(key: &[u8]) -> () {
    println!("deleting key :{:?}", key);
    
    ()
}