use crate::error::RedisError;
use crate::resp::{put_clrf, Resp, Serializer};
use bytes::{BufMut, BytesMut};

/// redis arrays
/// ` *<number-of-elements>\r\n<element-1>...<element-n> `
#[derive(Debug, Hash, Clone)]
pub(crate) struct Arrays(Vec<Resp>);

impl Serializer for Arrays {
    fn prefix() -> &'static str {
        "*"
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(Self::prefix().as_bytes());
        bytes.put_slice(self.0.len().to_string().as_bytes());
        put_clrf(&mut bytes);
        for e in &self.0 {
            let data = e.serialize()?;
            bytes.put_slice(data.as_slice());
        }
        Ok(bytes.to_vec())
    }
}
