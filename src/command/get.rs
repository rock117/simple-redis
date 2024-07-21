use std::fmt::Display;

use crate::command::Command;
use crate::context::Context;
use crate::error::RedisError;
use crate::resp::BulkStrings;
use crate::resp::Resp;
use crate::storage::mem::MemStorage;
use crate::storage::Storage;

/// redis get command
///
/// https://redis.io/docs/latest/commands/get/
pub struct Get {
    key: String,
}

impl Command for Get {
    fn name() -> &'static str {
        "Get"
    }

    fn args(&self) -> Vec<String> {
        vec![]
    }

    fn execute(
        &self,
        context: &dyn Context<Storage = MemStorage>,
    ) -> Result<Resp, RedisError> {
       todo!()
    }
}
impl TryFrom<BulkStrings> for Get {
    type Error = RedisError;

    fn try_from(value: BulkStrings) -> Result<Self, Self::Error> {
        let key = String::from_utf8(value.0).map_err(|e| RedisError::UnknowError(e.to_string()))?;
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
