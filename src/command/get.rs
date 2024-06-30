use std::fmt::Display;

use crate::command::Command;
use crate::context::Context;
use crate::error::RedisError;
use crate::resp::{AsResp, BulkStrings, Resp};
use crate::storage::mem::MemStorage;
use crate::storage::Storage;

/// redis get command
///
/// https://redis.io/docs/latest/commands/get/
pub struct Get {
    key: String,
}

impl Command for Get {
    fn execute(&self, context: &dyn Context<Storage = MemStorage>) -> Result<Resp, RedisError> {
        let storage = context.storage();
        let data = storage.get(&self.key).clone();
        match data {
            None => Err(RedisError::Other),
            Some(coll) => coll.as_resp_try(),
        }
    }
}
impl TryFrom<BulkStrings> for Get {
    type Error = RedisError;

    fn try_from(value: BulkStrings) -> Result<Self, Self::Error> {
        let key = String::from_utf8(value.0).unwrap(); // TODO
        Ok(Get { key })
    }
}
impl Display for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GET {}", self.key)
    }
}

fn test() {
    let bstr = BulkStrings(vec![]);
    let get = Get::try_from(bstr);
}
