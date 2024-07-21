pub(crate) mod arrays;
mod big_numbers;
mod bulk_errors;
pub mod bulk_strings;
mod integers;
mod maps;
pub(crate) mod nulls;
mod pushes;
pub(crate) mod simple_errors;
pub mod simple_strings;
mod verbatim_strings;
pub(crate) mod frame;

use crate::error::RedisError;
use crate::resp::arrays::Arrays;
pub(crate) use crate::resp::bulk_strings::BulkStrings;
use crate::resp::nulls::Nulls;
use crate::resp::simple_errors::SimpleErrors;
pub(crate) use crate::resp::simple_strings::SimpleStrings;
use bytes::{BufMut, BytesMut};
use serde::Serialize;
use std::str::FromStr;
use tracing::info;
use crate::resp::bulk_errors::BulkErrors;

/// https://redis.io/docs/latest/develop/reference/protocol-spec/
#[derive(Debug, Clone, Hash, strum::IntoStaticStr)]
pub enum Resp {
    SimpleStrings(SimpleStrings),
    SimpleErrors(SimpleErrors),
    BulkStrings(BulkStrings),
    BulkErrors(BulkErrors),
    Arrays(Arrays),
    Nulls(Nulls),
}

trait Serializer {
    fn prefix() -> &'static str;
    fn serialize(&self) -> Result<Vec<u8>, RedisError>;
}
trait DeSerializer where Self: Sized {
    fn deserialize(&self)-> Result<Self, RedisError>;
}

pub fn serialize(resp_frame: &Resp) -> Result<Vec<u8>, RedisError>  {
    info!("serializer resp: {:?}", resp_frame);
    match resp_frame {
        Resp::SimpleStrings(v) => v.serialize(),
        Resp::SimpleErrors(v) => v.serialize(),
        Resp::BulkStrings(v) => v.serialize(),
        Resp::Nulls(v) => v.serialize(),
        Resp::Arrays(v) => v.serialize(),
        _ => Err(RedisError::UnknowError("unknow resp type".into())),
    }
}

impl Resp {
    pub fn simple_strings(data: String) -> Resp {
        Resp::SimpleStrings(SimpleStrings(data))
    }
    pub fn simple_errors(err: String) -> Resp {
        Resp::SimpleErrors(SimpleErrors(err))
    }

    pub fn nulls() -> Resp {
        Resp::Nulls(Nulls)
    }
}

pub(self) fn put_clrf(bytes: &mut BytesMut) {
    bytes.put_slice(b"\r\n");
}

fn put_bulk_data(buf: &mut BytesMut, prefix: &[u8], data: &[u8]) {
    buf.put_slice(prefix);
    buf.put_slice(data.len().to_string().as_bytes());
    put_clrf(buf);
    buf.put_slice(data);
    put_clrf(buf);
}