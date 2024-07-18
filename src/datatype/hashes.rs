use crate::error::RedisError;
use crate::resp::{AsResp, RespFrame};

#[derive(Clone, Debug)]
pub struct Hashes;

impl AsResp for Hashes {
    fn as_resp_try(&self) -> Result<RespFrame, RedisError> {
        todo!()
    }
}
