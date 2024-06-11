use crate::error::RedisError;
use crate::resp::{put_clrf, Serializer, SimpleStrings};
use bytes::{BufMut, BytesMut};

/// redis Verbatim Strings
/// ` =<length>\r\n<encoding>:<data>\r\n `
pub(crate) struct VerbatimStrings {
    encoding: String,
    data: Vec<u8>,
}

impl Serializer for VerbatimStrings {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(b"=");
        bytes.put_slice(self.data.len().to_string().as_bytes());
        put_clrf(&mut bytes);
        bytes.put_slice(self.encoding.as_bytes());
        bytes.put_slice(b":");
        bytes.put_slice(self.data.as_slice());
        put_clrf(&mut bytes);
        Ok(bytes.to_vec())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serialize() {
        let vstr = VerbatimStrings {
            encoding: "txt".into(),
            data: b"hello".to_vec(),
        };
        assert_eq!(
            "=5\r\ntxt:hello\r\n",
            String::from_utf8(vstr.serialize().unwrap()).unwrap()
        );
    }
}
