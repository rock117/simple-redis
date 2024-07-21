use std::pin::Pin;
use crate::command::Command;
use crate::context::Context;
use crate::error::RedisError;
use crate::resp::Resp;
use crate::storage::mem::MemStorage;

pub(crate) struct Ping(Option<String>);

impl Command for Ping {
    fn name() -> &'static str {
        "Ping"
    }

    fn args(&self) -> Vec<String> {
        self.0.clone().into_iter().collect()
    }

    fn execute(&self, context: &dyn Context<Storage=MemStorage>) -> Result<Resp, RedisError> {
        todo!()
    }
}
