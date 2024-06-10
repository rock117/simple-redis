use std::fmt::Display;

/// redis get command
///
/// https://redis.io/docs/latest/commands/get/
pub struct Get {
    pub key: String,
}

impl Display for Get {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GET {}", self.key)
    }
}
