use crate::command::get::Get;
use crate::context::Context;
use crate::error::RedisError;
use crate::resp::{BulkStrings, RespFrame};
use crate::storage::mem::MemStorage;

mod append;
mod del;
mod exist;
mod expire;
mod get;
mod hello;
pub mod parser;
mod ping;
mod set;

pub trait Command: Send {
    fn execute(&self, context: &dyn Context<Storage = MemStorage>)
        -> Result<RespFrame, RedisError>;
}

pub fn parse_command(bulk_strings: BulkStrings) -> Option<Box<dyn Command>> {
    if bulk_strings.len() == 0 {
        return None;
    }
    match bulk_strings.inner_ref() {
        s if s.starts_with(b"GET") => {
            let cmd = Get::try_from(bulk_strings).unwrap();
            Some(Box::new(cmd))
        }
        _ => None,
    }
}
