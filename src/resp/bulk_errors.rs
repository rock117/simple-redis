use crate::error::RedisError;
use crate::resp::{put_bulk_data, put_clrf, Serializer};
use bytes::{BufMut, BytesMut};

/// redis bulk errors
/// ` !<length>\r\n<error>\r\n `
#[derive(Clone, Debug, Hash)]
pub(crate) struct BulkErrors(Vec<u8>);

impl Serializer for BulkErrors {
    fn prefix() -> &'static str {
        "!"
    }
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        put_bulk_data(&mut bytes, Self::prefix().as_bytes(), self.0.as_slice());
        Ok(bytes.to_vec())
    }
}
