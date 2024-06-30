use nonempty_collections::{NESet, NonEmptyIterator};
use std::fmt::Display;

/// redis del command
///
/// https://redis.io/docs/latest/commands/del/
pub(crate) struct Del {
    keys: NESet<String>,
}

impl Display for Del {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self.keys.clone().into_iter().collect::<Vec<String>>();
        let key_strs = keys.join(" ");
        write!(f, "DEL {}", key_strs)
    }
}
