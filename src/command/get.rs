use std::fmt::Display;

use crate::command::Command;
use crate::context::Context;
use crate::error::RedisError;
use crate::resp::Resp;
use crate::storage::Storage;

/// redis get command
///
/// https://redis.io/docs/latest/commands/get/
pub struct Get {
    key: String,
}

impl Command for Get {
    fn execute<T: Context>(&self, context: &T) -> Result<Resp, RedisError> {
        let storage = context.storage();
        let data = storage.get(&self.key).clone();
        match data {
            Some(data) => Ok(data),
            None => Ok(Resp::Nulls),
        }
    }
}

impl Display for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GET {}", self.key)
    }
}
