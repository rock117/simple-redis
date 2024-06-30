use crate::error::RedisError;
use crate::resp::{Resp, Serializer};
use bytes::{BufMut, BytesMut};
use std::collections::HashMap;

/// redis Maps
/// ` %<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n> `
pub(crate) struct Maps(HashMap<Resp, Resp>);

impl Serializer for Maps {
    fn prefix() -> &'static str {
        "%"
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(Self::prefix().as_bytes());
        bytes.put_slice(self.0.len().to_string().as_bytes());
        for (k, v) in &self.0 {
            bytes.put_slice(k.serialize()?.as_slice());
            bytes.put_slice(v.serialize()?.as_slice());
        }
        Ok(bytes.to_vec())
    }
}
