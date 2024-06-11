use crate::command::get::Get;
use crate::command::parser::CommandParser;
use crate::error::RedisError;
use crate::resp::BulkStrings;

pub struct GetParser;

impl CommandParser for GetParser {
    type Command = Get;

    fn parse(str: &BulkStrings) -> Result<Self::Command, RedisError> {
        todo!()
    }
}
