use crate::datatype::RedisCollection;

mod file;
pub mod mem;

pub trait Storage {
    fn put(&self, key: String, value: RedisCollection);
    fn get(&self, key: &str) -> Option<RedisCollection>;

    fn remove(&self, key: &str);
}
