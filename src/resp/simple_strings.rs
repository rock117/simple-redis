use crate::error::RedisError;
use crate::resp::{put_clrf, Serializer};
use bytes::{BufMut, BytesMut};

/// redis simple strings
/// ``` +OK\r\n ```
#[derive(Debug, Hash, Clone)]
pub struct SimpleStrings(pub String);

impl Serializer for SimpleStrings {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::with_capacity(self.0.len() + 3);
        bytes.put_slice(b"+");
        bytes.put_slice(self.0.as_bytes());
        put_clrf(&mut bytes);
        Ok(bytes.to_vec())
    }
}
