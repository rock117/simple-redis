use std::fmt::Display;

/// redis del command
///
/// https://redis.io/docs/latest/commands/del/
pub(crate) struct Del {
    keys: Vec<String>,
}

impl Display for Del {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_strs = self.keys.join(" ");
        write!(f, "GET {}", key_strs)
    }
}
