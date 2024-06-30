use crate::error::RedisError;
use crate::resp::bulk_strings;
use crate::resp::Resp::BulkStrings;
use crate::resp::{Resp, Serializer};
use bytes::{Buf, BufMut, BytesMut};
use nom::AsBytes;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::is_digit;
use nom::sequence::tuple;
use tokio_util::codec::{Decoder, Encoder};
use tracing::{error, info};

pub struct RespCodec;

impl Encoder<Resp> for RespCodec {
    type Error = RedisError;

    fn encode(&mut self, item: Resp, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = item.serialize().unwrap(); // TODO
        dst.put_slice(bytes.as_slice());
        Ok(())
    }
}

impl Decoder for RespCodec {
    type Item = Resp;
    type Error = RedisError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let data = src.as_bytes();

     //   let v = nom::bytes::complete::take(1usize)(src);

       // let arr = tuple((tag("*"), take_while1(is_digit), tag("\r\n")))(data);
     //   let d = src.clone();
     //   let dd = d.to_vec();
        //info!("receive data: {}", String::from_utf8_lossy(dd.as_slice()));
        match src.first() {
            Some(b'*') => {}
            Some(b'$') => {}
            Some(first) => {
                return Err(RedisError::UnknowError(format!(
                    "not support resp prefix: {}",
                    String::from_utf8_lossy(&[*first])
                )))
            }
            None => return Err(RedisError::UnknowError("no value in src".into())),
        }

        src.advance(1);
        let len = read_len(src).map(|v| v as usize);
        let Some(len) = len else {
            return Err(RedisError::UnknowError(
                "bulkstrings len parse failed".into(),
            ));
        };
        if src.len() < len {
            info!(
                "not enough data to read, expect {}, actual {}",
                len,
                src.len()
            );
            return Ok(None);
        }
        let (data, _) = src.split_at(len);
        let bulk_str = bulk_strings::BulkStrings(data.to_vec());
        src.advance(len);
        if src.starts_with(b"\r\n") {
            src.advance(2);
            Ok(Some(BulkStrings(bulk_str)))
        } else {
            Ok(None)
        }
    }
}

fn read_len(src: &mut BytesMut) -> Option<u64> {
    // TODO not correct
    let mut i = 0;
    let mut len = 0usize;
    while let Some(v) = src.get(i) {
        let v = *v;
        if v >= b'0' && v <= b'9' {
            len += 1;
        }
        i += 1;
    }
    if len < 1 {
        return None;
    }
    Some(src.get_uint(len))
}
