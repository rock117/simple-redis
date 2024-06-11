use dashmap::DashMap;

use crate::error::RedisError;
use crate::resp::Resp;
use crate::storage::Storage;

pub struct MemStorage(DashMap<String, Resp>);
impl Storage for MemStorage {
    fn put(&self, key: String, value: Resp) {
        self.0.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<Resp> {
        self.0.get(key).map(|v| v.clone())
    }

    fn remove(&self, key: &str) {
        self.0.remove(key);
    }
}

pub fn get(key: &str) -> Result<Resp, RedisError> {
    todo!()
}
