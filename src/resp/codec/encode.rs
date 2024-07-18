use bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::error::RedisError;
use crate::resp::{Resp, RespCodec, Serializer};

impl Encoder<Resp> for RespCodec {
    type Error = RedisError;

    fn encode(&mut self, item: Resp, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = item.serialize()?;
        dst.extend_from_slice(&bytes);
        Ok(())
    }
}
