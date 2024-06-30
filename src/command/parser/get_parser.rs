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
trait CommandParser2
where
    Self: Sized,
{
    fn parse(str: &BulkStrings) -> Result<Self, RedisError>;
}
impl CommandParser2 for Get {
    fn parse(str: &BulkStrings) -> Result<Self, RedisError> {
        todo!()
    }
}
