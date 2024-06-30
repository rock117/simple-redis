mod get_parser;
mod set_parser;

use crate::command::Command;
use crate::error::RedisError;
use crate::resp::BulkStrings;
pub use get_parser::GetParser;
pub use set_parser::SetParser;

pub trait CommandParser {
    type Command: Command;
    fn parse(str: &BulkStrings) -> Result<Self::Command, RedisError>;
}

// pub(crate) fn parse(data: &str) -> impl Command {
//     unimplemented!()
// }
