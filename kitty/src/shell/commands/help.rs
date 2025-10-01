use crate::shell::command::{CommandArgs, CommandNames, ShellCommand};
use crate::shell::frontend::Shell;
use anyhow::{Result, bail};

pub struct HelpCommand;

impl ShellCommand for HelpCommand {
    fn names(&self) -> CommandNames {
        &["help"]
    }

    fn description(&self) -> &str {
        "kitty help tool"
    }

    fn help(&self) -> &str {
        "Provides help for kitty.\n\
        Usage: help [command]"
    }

    fn run(&self, shell: &mut Shell, args: CommandArgs) -> Result<()> {
        if args.len() > 1 {
            bail!("Invalid usage");
        }

        if let Some(cmd) = args.first() {
            for command in shell.commands().iter() {
                if !command.can_handle(cmd) {
                    continue;
                }

                println!("Command: {}", command.names().join(", "));
                println!("Description: {}", command.description());
                println!("{}", command.help());
                break;
            }
        } else {
            println!("Available Commands:");
            for command in shell.commands().iter() {
                let names = command.names();
                let help = command.description();
                if help.is_empty() {
                    continue;
                }
                println!("- {}: {}", names.join(", "), command.description());
            }
        }
        Ok(())
    }
}
