use dashmap::DashMap;

use crate::datatype::RedisCollection;
use crate::storage::Storage;

pub struct MemStorage(DashMap<String, RedisCollection>);
impl Storage for MemStorage {
    fn put(&self, key: String, value: RedisCollection) {
        self.0.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<RedisCollection> {
        self.0.get(key).map(|v| v.clone())
    }

    fn remove(&self, key: &str) {
        self.0.remove(key);
    }
}
