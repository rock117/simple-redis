use crate::command::get::Get;
use crate::command::parser::CommandParser;
use crate::error::RedisError;
use crate::resp::BulkStrings;
use std::process::Command;

struct GetParser;
impl CommandParser for GetParser {
    type Command = Get;

    fn parse(str: &BulkStrings) -> Result<Command, RedisError> {
        todo!()
    }
}
