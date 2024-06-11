mod arrays;
mod big_numbers;
mod bulk_errors;
pub mod bulk_strings;
mod codec;
mod integers;
mod maps;
mod nulls;
mod pushes;
mod simple_errors;
pub mod simple_strings;
mod verbatim_strings;

use crate::error::RedisError;
use crate::resp::arrays::Arrays;
pub(crate) use crate::resp::bulk_strings::BulkStrings;
pub(crate) use crate::resp::codec::RespCodec;
use crate::resp::simple_errors::SimpleErrors;
pub(crate) use crate::resp::simple_strings::SimpleStrings;
use bytes::{BufMut, BytesMut};
use serde::Serialize;
use std::str::FromStr;

/// https://redis.io/docs/latest/develop/reference/protocol-spec/
#[derive(Debug, Clone, Hash)]
pub enum Resp {
    SimpleStrings(SimpleStrings),
    SimpleErrors(SimpleErrors),
    BulkStrings(BulkStrings),
    Arrays(Arrays),
    Nulls,
}

trait Serializer {
    fn serialize(&self) -> Result<Vec<u8>, RedisError>;
}

trait RespFirstByte {
    fn first() -> u8;
}

impl Serializer for Resp {
    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        match self {
            Resp::SimpleStrings(v) => v.serialize().unwrap(), // TODO
            Resp::SimpleErrors(v) => v.serialize().unwrap(),
            Resp::BulkStrings(v) => v.serialize().unwrap(),
            _ => unreachable!(),
        };
        Ok(vec![])
    }
}

impl FromStr for Resp {
    type Err = RedisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub(self) fn put_clrf(bytes: &mut BytesMut) {
    bytes.put_slice(b"\r\n");
}
