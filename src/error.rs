use crate::error::RedisError::IOError;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisError {
    CommandParseError(CommandParseError),
    IOError(std::io::Error),
}

#[derive(Clone, Debug)]
pub(crate) enum CommandParseError {
    IllegalCommandArgs { cmd: String, msg: String },
    NotSupportCommand(String),
}

impl From<std::io::Error> for RedisError {
    fn from(value: std::io::Error) -> Self {
        IOError(value)
    }
}
impl Display for RedisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
