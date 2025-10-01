use crate::shell::command::{CommandArgs, CommandNames, ShellCommand};
use crate::shell::frontend::Shell;
use crate::utils::is_booted_natos;
use anyhow::{Result, bail};
use std::process::Command;

pub struct PurrCommand;

impl ShellCommand for PurrCommand {
    fn names(&self) -> CommandNames {
        &["purr"]
    }

    fn description(&self) -> &str {
        "Render Purr."
    }

    fn help(&self) -> &str {
        "Render Purr.\n\
        Usage: purr [list|image]"
    }

    fn run(&self, _shell: &mut Shell, args: CommandArgs) -> Result<()> {
        if !is_booted_natos() {
            bail!("Not booted into natOS");
        }

        let status = Command::new("/bin/purr").args(args).status()?;
        if !status.success() {
            println!("purr failed: {}", status.code().unwrap_or(-1));
        }
        Ok(())
    }
}
