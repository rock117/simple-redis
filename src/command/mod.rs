use crate::context::Context;
use crate::error::RedisError;
use crate::resp::Resp;

mod append;
mod del;
mod exist;
mod expire;
mod get;
mod hello;
pub mod parser;
mod ping;
mod set;

pub trait Command {
    fn execute<T: Context>(&self, context: &T) -> Result<Resp, RedisError>;
}
