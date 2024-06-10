use crate::error::RedisError;
use crate::resp::{Resp, Serializer};
use bytes::{BufMut, BytesMut};
use nom::AsBytes;

/// redis arrays
/// ` *<number-of-elements>\r\n<element-1>...<element-n> `
#[derive(Debug, Hash)]
pub(crate) struct Arrays(Vec<Resp>);

impl Serializer for Arrays {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(b"*");
        bytes.put_slice(self.0.len().to_string().as_bytes());
        bytes.put_slice(b"\r\n");
        for e in &self.0 {
            let data = e.serialize()?;
            bytes.put_slice(data.as_bytes());
        }
        Ok(bytes.to_vec())
    }
}
