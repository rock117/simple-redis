use std::str::FromStr;

/// https://redis.io/docs/latest/develop/reference/protocol-spec/
pub enum Resp {
    SimpleStrings(String),
    SimpleErrors(String),
}

impl FromStr for Resp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
