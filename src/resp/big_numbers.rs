use bytes::{BufMut, BytesMut};
use num::{BigInt, Zero};

use crate::error::RedisError;
use crate::resp::{put_clrf, Serializer};

/// redis big numbers
/// ` ([+|-]<number>\r\n `
pub(crate) struct BigNumbers(BigInt);

impl Serializer for BigNumbers {
    fn prefix() -> &'static str {
        "("
    }

    fn serialize(&self) -> Result<Vec<u8>, RedisError> {
        let mut bytes = BytesMut::new();
        let sign = if self.0 < BigInt::zero() { "-" } else { "" };
        bytes.put_slice(Self::prefix().as_bytes());
        bytes.put_slice(sign.as_bytes());
        bytes.put_slice(self.0.to_string().as_bytes());
        put_clrf(&mut bytes);
        Ok(bytes.to_vec())
    }
}
