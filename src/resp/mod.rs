mod bulk_strings;
mod simple_strings;
mod simple_errors;

use crate::error::RedisError;
use std::str::FromStr;
pub(crate) use crate::resp::bulk_strings::BulkStrings;
use crate::resp::simple_errors::SimpleErrors;
use crate::resp::simple_strings::SimpleStrings;

/// https://redis.io/docs/latest/develop/reference/protocol-spec/
#[derive(Debug)]
pub enum Resp {
    SimpleStrings(SimpleStrings),
    SimpleErrors(SimpleErrors),
    BulkStrings(BulkStrings),
}

trait Serializer {
    fn serialize(&self) -> Result<Vec<u8>, RedisError>;
}

impl Serializer for Resp {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        match self {
            Resp::SimpleStrings(_) => {}
            Resp::SimpleErrors(_) => {}
            Resp::BulkStrings(_) => {}
        }
        Ok(vec![])
    }
}

impl FromStr for Resp {
    type Err = RedisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}


