use crate::resp::Resp;

mod file;
pub mod mem;
pub use mem::MemStorage;

pub trait Storage {
    fn put(&self, key: String, value: Resp);
    fn get(&self, key: &str) -> Option<Resp>;

    fn remove(&self, key: &str);
}
