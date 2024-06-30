use crate::error::RedisError;
use crate::resp::{AsResp, Resp};

#[derive(Clone, Debug)]
pub struct Hashes;

impl AsResp for Hashes {
    fn as_resp_try(&self) -> Result<Resp, RedisError> {
        todo!()
    }
}
