use crate::error::RedisError;
use crate::resp::Resp;
use crate::storage::Storage;

pub(crate) struct MemStorage;
impl Storage for MemStorage {}

pub fn get(key: &str) -> Result<Resp, RedisError> {
    todo!()
}
