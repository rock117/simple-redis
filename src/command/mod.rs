use crate::command::del::Del;
use crate::command::get::Get;
use crate::command::set::Set;
use crate::error::RedisError;
use crate::resp::Resp;

mod del;
mod get;
mod parser;
mod set;

pub enum RedisCommand {
    Get(Get),
    Set(Set),
    Del(Del),
}

impl RedisCommand {
    pub fn execute(&self) -> Result<Resp, RedisError> {
        todo!()
    }
}

pub fn execute() {}
