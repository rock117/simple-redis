use bytes::{BufMut, BytesMut};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::RedisError;
use crate::resp::{put_bulk_data, put_clrf, Serializer};

/// redis bulk stings:
///
///``` $<length>\r\n<data>\r\n ```
#[derive(Debug, Hash, Clone)]
pub struct BulkStrings(pub Vec<u8>);

impl BulkStrings {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn inner_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Serializer for BulkStrings {
    fn prefix() -> &'static str {
        "$"
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        put_bulk_data(&mut bytes, Self::prefix().as_bytes(), self.0.as_slice());
        Ok(bytes.to_vec())
    }
}

impl Display for BulkStrings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for BulkStrings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
