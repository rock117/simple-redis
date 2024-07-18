use crate::error::RedisError::IOError;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisError {
    IOError(std::io::Error),
    RespError(RespError),
    Other,
    UnknowError(String),
}
#[derive(Debug, Error)]
pub enum RespError {
    InvalidResp,
    InComplete,
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

impl Display for RespError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
