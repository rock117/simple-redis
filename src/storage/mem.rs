use crate::error::RedisError;
use crate::resp::Resp;
use crate::storage::Storage;
use dashmap::DashMap;

pub(crate) struct MemStorage<V>(DashMap<String, V>);
impl<V> Storage for MemStorage<V> {
    fn put<T>(key: String, value: T) {
        todo!()
    }

    fn get<T>(key: &str) -> Option<T> {
        todo!()
    }

    fn remove(key: &str) {
        todo!()
    }
}

pub fn get(key: &str) -> Result<Resp, RedisError> {
    todo!()
}
