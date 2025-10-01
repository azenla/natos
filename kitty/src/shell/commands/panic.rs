use crate::shell::command::{CommandArgs, CommandNames, ShellCommand};
use crate::shell::frontend::Shell;
use anyhow::{Result, bail};

pub struct PanicCommand;

impl ShellCommand for PanicCommand {
    fn names(&self) -> CommandNames {
        &["panic"]
    }

    fn description(&self) -> &str {
        "Panic the system."
    }

    fn help(&self) -> &str {
        "Panic the system.\n\
        Usage: panic"
    }

    fn run(&self, _shell: &mut Shell, args: CommandArgs) -> Result<()> {
        if !args.is_empty() {
            bail!("Invalid usage");
        }
        panic!("a dinosaur was here");
    }
}
