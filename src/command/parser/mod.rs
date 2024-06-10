mod get_parser;

use crate::command::RedisCommand;
use crate::error::RedisError;
use crate::resp::BulkStrings;
use std::process::Command;

trait CommandParser {
    type Command;
    fn parse(str: &BulkStrings) -> Result<Command, RedisError>;
}

pub(crate) fn parse(data: &str) -> Result<RedisCommand, RedisError> {
    todo!()
}
