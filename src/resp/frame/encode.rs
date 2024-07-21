use bytes::BytesMut;
use tokio_util::codec::Encoder;

use crate::error::RedisError;
use crate::resp::frame::RespCodec;
use crate::resp::Resp;

impl Encoder<Resp> for RespCodec {
    type Error = RedisError;

    fn encode(&mut self, item: Resp, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = crate::resp::serialize(&item)?;
        dst.extend_from_slice(&bytes);
        Ok(())
    }
}
