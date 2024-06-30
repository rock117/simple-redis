use std::collections::HashSet;
use std::hash::Hash;

use crate::error::RedisError;
use crate::resp::{AsResp, Resp};

#[derive(Clone, Debug)]
pub struct Sets(HashSet<String>);

impl AsResp for Sets {
    fn as_resp_try(&self) -> Result<Resp, RedisError> {
        todo!()
    }
}
