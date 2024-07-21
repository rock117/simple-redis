use crate::context::Context;
use crate::error::RedisError;
use crate::resp::Resp;
use crate::storage::mem::MemStorage;

mod append;
mod del;
mod exist;
mod expire;
mod get;
mod hello;
mod ping;
mod set;
mod command_executor;

pub trait Command: Send {
    fn name() -> &'static str where Self: Sized;
    fn args(&self) -> Vec<String>;
    fn execute(&self, context: &dyn Context<Storage = MemStorage>)
        -> Result<Resp, RedisError>;
}
