use crate::error::RedisError;
use crate::resp::{put_clrf, Serializer};
use bytes::{BufMut, BytesMut};

/// redis simple errors
/// ``` -Error message\r\n ```
#[derive(Debug, Hash, Clone)]
pub(crate) struct SimpleErrors(pub String);

impl Serializer for SimpleErrors {
    fn prefix() -> &'static str {
        "-"
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(Self::prefix().as_bytes());
        bytes.put_slice(self.0.as_bytes());
        put_clrf(&mut bytes);
        Ok(bytes.to_vec())
    }
}
