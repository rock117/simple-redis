use bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::error::RedisError;
use crate::resp::{RespCodec, RespFrame, Serializer};

impl Encoder<RespFrame> for RespCodec {
    type Error = RedisError;

    fn encode(&mut self, item: RespFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = item.serialize()?;
        dst.extend_from_slice(&bytes);
        Ok(())
    }
}
