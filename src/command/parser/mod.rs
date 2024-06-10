mod get_parser;

use crate::command::RedisCommand;
use crate::error::RedisError;
use std::process::Command;
use crate::resp::BulkStrings;

trait CommandParser {
    type Command;
    fn parse(str: &BulkStrings) -> Result<Command, RedisError>;
}

pub(crate) fn parse(data: &str) -> Result<RedisCommand, RedisError> {
    todo!()
}
