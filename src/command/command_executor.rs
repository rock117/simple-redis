use crate::command::Command;
use crate::context::Context;

pub(crate) struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(cmd: &impl Command, context: &impl Context) {
        // find redis type its method by command name
        // LPUSH -> list.lpush
    }
}