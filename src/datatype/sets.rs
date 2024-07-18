use std::collections::HashSet;
use std::hash::Hash;

use crate::error::RedisError;
use crate::resp::{AsResp, RespFrame};

#[derive(Clone, Debug)]
pub struct Sets(HashSet<String>);

impl AsResp for Sets {
    fn as_resp_try(&self) -> Result<RespFrame, RedisError> {
        todo!()
    }
}
