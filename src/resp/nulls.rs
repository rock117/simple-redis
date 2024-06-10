use crate::error::RedisError;
use crate::resp::Serializer;
use bytes::Bytes;

/// redis simple errors
/// ``` -Error message\r\n ```
#[derive(Debug)]
pub(crate) struct Nulls;

impl Serializer for Nulls {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        Ok(Vec::from(b"_\r\n"))
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
