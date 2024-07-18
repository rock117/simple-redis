mod arrays;
mod big_numbers;
mod bulk_errors;
pub mod bulk_strings;
mod codec;
mod integers;
mod maps;
pub(crate) mod nulls;
mod pushes;
pub(crate) mod simple_errors;
pub mod simple_strings;
mod verbatim_strings;

use crate::error::RedisError;
use crate::resp::arrays::Arrays;
pub(crate) use crate::resp::bulk_strings::BulkStrings;
pub(crate) use crate::resp::codec::RespCodec;
use crate::resp::nulls::Nulls;
use crate::resp::simple_errors::SimpleErrors;
pub(crate) use crate::resp::simple_strings::SimpleStrings;
use bytes::{BufMut, BytesMut};
use serde::Serialize;
use std::str::FromStr;
use tracing::info;

/// https://redis.io/docs/latest/develop/reference/protocol-spec/
#[derive(Debug, Clone, Hash, strum::IntoStaticStr)]
pub enum RespFrame {
    SimpleStrings(SimpleStrings),
    SimpleErrors(SimpleErrors),
    BulkStrings(BulkStrings),
    Arrays(Arrays),
    Nulls(Nulls),
}

pub trait AsResp {
    fn as_resp_try(&self) -> Result<RespFrame, RedisError>;
}

trait Serializer {
    fn prefix() -> &'static str;
    fn serialize(&self) -> Result<Vec<u8>, RedisError>;
}

impl Serializer for RespFrame {
    fn prefix() -> &'static str {
        "" // TODO
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        info!("serializer resp: {:?}", self);
        match self {
            RespFrame::SimpleStrings(v) => v.serialize(),
            RespFrame::SimpleErrors(v) => v.serialize(),
            RespFrame::BulkStrings(v) => v.serialize(),
            RespFrame::Nulls(v) => v.serialize(),
            RespFrame::Arrays(v) => v.serialize(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for RespFrame {
    type Err = RedisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub(self) fn put_clrf(bytes: &mut BytesMut) {
    bytes.put_slice(b"\r\n");
}
