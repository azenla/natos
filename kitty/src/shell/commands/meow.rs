use crate::shell::command::{CommandArgs, CommandNames, ShellCommand};
use crate::shell::frontend::Shell;
use anyhow::{Result, bail};

pub struct MeowCommand;

impl ShellCommand for MeowCommand {
    fn names(&self) -> CommandNames {
        &["meow"]
    }

    fn description(&self) -> &str {
        "meow"
    }

    fn help(&self) -> &str {
        "Meow.\n\
        Usage: meow"
    }

    fn run(&self, _shell: &mut Shell, args: CommandArgs) -> Result<()> {
        if !args.is_empty() {
            bail!("Invalid usage");
        }
        println!("meow");
        Ok(())
    }
}
