use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// redis bulk stings:
///
///``` $<length>\r\n<data>\r\n ```
#[derive(Debug)]
pub struct BulkStrings {
    len: usize,
    data: Vec<u8>
}

impl Display for BulkStrings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for BulkStrings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}