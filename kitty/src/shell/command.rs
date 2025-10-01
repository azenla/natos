use crate::shell::frontend::Shell;
use anyhow::Result;

pub type CommandNames<'a> = &'a [&'a str];
pub type CommandArgs<'a> = &'a [&'a str];
pub type CommandList = Vec<Box<dyn ShellCommand>>;

pub trait ShellCommand {
    fn names(&self) -> CommandNames;

    fn can_handle(&self, name: &str) -> bool {
        self.names().contains(&name)
    }

    fn description(&self) -> &str {
        "description not provided"
    }

    fn help(&self) -> &str {
        "help not provided"
    }

    fn run(&self, shell: &mut Shell, args: CommandArgs) -> Result<()>;
}
