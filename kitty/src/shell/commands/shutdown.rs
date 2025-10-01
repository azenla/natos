use crate::shell::command::{CommandArgs, CommandNames, ShellCommand};
use crate::shell::frontend::Shell;
use crate::utils::is_booted_natos;
use anyhow::{Result, bail};

pub struct ShutdownCommand;

impl ShellCommand for ShutdownCommand {
    fn names(&self) -> CommandNames {
        &["shutdown", "poweroff", "exit"]
    }

    fn description(&self) -> &str {
        "Shutdown the system."
    }

    fn help(&self) -> &str {
        "Shutdown the system.\n\
        Usage: shutdown"
    }

    fn run(&self, _shell: &mut Shell, args: CommandArgs) -> Result<()> {
        if !args.is_empty() {
            bail!("Invalid usage");
        }

        if cfg!(target_os = "linux") && is_booted_natos() {
            #[cfg(target_os = "linux")]
            if unsafe { libc::reboot(libc::RB_POWER_OFF) } != 0 {
                bail!("failed to shutdown");
            }
        } else {
            std::process::exit(0);
        }

        Ok(())
    }
}
