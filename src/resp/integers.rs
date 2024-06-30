use crate::error::RedisError;
use crate::resp::{put_clrf, Serializer};
use bytes::{BufMut, BytesMut};

/// redis Integers
/// ``` :[<+|->]<value>\r\n ```
pub(crate) struct Integers(i64);
impl Serializer for Integers {
    fn prefix() -> &'static str {
        ":"
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let sign = if self.0 < 0 { "-" } else { "" };
        let mut bytes = BytesMut::new();
        bytes.put_slice(Self::prefix().as_bytes());
        bytes.put_slice(sign.as_bytes());
        bytes.put_slice(self.0.to_string().as_bytes());
        put_clrf(&mut bytes);
        Ok(bytes.to_vec())
    }
}
