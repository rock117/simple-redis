use crate::command::del::Del;
use crate::command::get::Get;
use crate::command::set::Set;

mod del;
mod get;
mod parser;
mod set;

pub enum RedisCommand {
    Get(Get),
    Set(Set),
    Del(Del),
}
