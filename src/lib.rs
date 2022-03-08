/// re-impl leveldb in rust
/// 
/// 

mod db_impl;
pub mod status;

pub struct DB {

}

/// impl DB basic ops
impl DB {
    pub fn open() -> Self {
        println!("#Open DB");

        DB {

        }
    }

    pub fn close(&self) {
        println!("#Close DB");
    }
}

/// impl put / get / delete
impl DB {
    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        println!("key : {:?}", key.as_ref());
        println!("value : {:?}", value.as_ref());
    }

    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        db_impl::_get(key)
    }

    pub fn delete(&mut self, key: &[u8]) -> () {
        println!("deleting key :{:?}", key);

        ()
    }
}
