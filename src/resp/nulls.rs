use crate::error::RedisError;
use crate::resp::Serializer;
use bytes::{BufMut, Bytes, BytesMut};

/// redis null
/// ``` _\r\n ```
#[derive(Debug, Copy, Clone, Hash)]
pub(crate) struct Nulls;

impl Serializer for Nulls {
    fn prefix() -> &'static str {
        "_"
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(Self::prefix().as_bytes());
        bytes.put_slice(b"\r\n");
        Ok(bytes.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::nulls::*;

    #[test]
    fn test_serialize() {
        let nulls = Nulls;
        assert_eq!(
            "_\r\n",
            String::from_utf8(nulls.serialize().unwrap()).unwrap()
        )
    }
}
