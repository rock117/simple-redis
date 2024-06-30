use crate::error::RedisError;
use crate::resp::{AsResp, Resp};

#[derive(Clone, Debug)]
pub struct Lists;

impl Lists {
    pub fn lpush(&mut self) {}
    pub fn lpop(&mut self) {}
    pub fn llen(&mut self) {}

    pub async fn blpop(&mut self) {}
    pub async fn blmove(&mut self, another: &mut Lists) {}
}

impl AsResp for Lists {
    fn as_resp_try(&self) -> Result<Resp, RedisError> {
        todo!()
    }
}
