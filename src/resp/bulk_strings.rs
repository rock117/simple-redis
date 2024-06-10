use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::RedisError;
use crate::resp::Serializer;

/// redis bulk stings:
///
///``` $<length>\r\n<data>\r\n ```
#[derive(Debug, Hash)]
pub struct BulkStrings {
    len: usize,
    data: Vec<u8>,
}

impl Serializer for BulkStrings {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        todo!()
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
