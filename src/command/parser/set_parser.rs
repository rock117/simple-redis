use crate::command::parser::CommandParser;
use crate::command::set::Set;
use crate::error::RedisError;
use crate::resp::BulkStrings;

pub struct SetParser;

impl CommandParser for SetParser {
    type Command = Set;

    fn parse(str: &BulkStrings) -> Result<Self::Command, RedisError> {
        todo!()
    }
}
