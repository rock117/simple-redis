use bytes::{BufMut, BytesMut};
use crate::error::RedisError;
use crate::resp::Serializer;

/// redis simple strings
/// ``` +OK\r\n ```
#[derive(Debug)]
pub(crate) struct SimpleStrings(String);

impl Serializer for SimpleStrings {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::with_capacity(self.0.len());
        bytes.put_slice(b"+");
        bytes.put_slice(self.0.as_bytes());
        bytes.put_slice(b"\r\n");
        Ok(bytes.to_vec())
    }
}