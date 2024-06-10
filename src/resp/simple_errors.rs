use crate::error::RedisError;
use crate::resp::Serializer;

/// redis simple errors
/// ``` -Error message\r\n ```
#[derive(Debug, Hash)]
pub(crate) struct SimpleErrors(String);

impl Serializer for SimpleErrors {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        todo!()
    }
}
