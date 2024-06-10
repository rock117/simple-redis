use crate::error::RedisError;
use crate::resp::Serializer;
use bytes::{BufMut, BytesMut};

/// redis Integers
/// ``` :[<+|->]<value>\r\n ```
pub(crate) struct Integers(i64);
impl Serializer for Integers {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let sign = if self.0 < 0 { "-" } else { "" };
        let mut bytes = BytesMut::new();
        bytes.put_slice(b":");
        bytes.put_slice(sign.as_bytes());
        bytes.put_slice(self.0.to_string().as_bytes());
        bytes.put_slice(b"\r\n");
        Ok(bytes.to_vec())
    }
}
