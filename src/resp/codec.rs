use crate::error::RedisError;
use crate::resp::{Resp, Serializer};
use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
pub struct RespCodec;

impl Encoder<Resp> for RespCodec {
    type Error = RedisError;

    fn encode(&mut self, item: Resp, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = item.serialize().unwrap(); // TODO
        dst.put_slice(bytes.as_slice());
        todo!()
    }
}

impl Decoder for RespCodec {
    type Item = Resp;
    type Error = RedisError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
