use crate::error::RedisError;
use crate::resp::{put_clrf, Serializer};
use bytes::{BufMut, BytesMut};

/// redis bulk errors
/// ` !<length>\r\n<error>\r\n `
pub(crate) struct BulkErrors(Vec<u8>);

impl Serializer for BulkErrors {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        bytes.put_slice(b"!");
        bytes.put_slice(self.0.len().to_string().as_bytes());
        put_clrf(&mut bytes);
        bytes.put_slice(self.0.as_slice());
        put_clrf(&mut bytes);
        Ok(bytes.to_vec())
    }
}
