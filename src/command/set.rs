use std::fmt::Display;
/// redis set command
///
/// https://redis.io/docs/latest/commands/set/
pub struct Set {
    key: String,
    value: String,
    ex: Option<usize>,
    exat: Option<usize>,
    pxat: Option<usize>,
    nx: bool,
    xx: bool,
    get: bool,
    keepttl: bool,
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SET {} {}", self.key, self.value) // TODO
    }
}
