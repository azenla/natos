use crate::shell::command::{CommandArgs, CommandNames, ShellCommand};
use crate::shell::frontend::Shell;
use crate::utils::is_booted_natos;
use anyhow::{Result, bail};
use std::ffi::CString;
use std::str::FromStr;

pub struct RebootCommand;

impl ShellCommand for RebootCommand {
    fn names(&self) -> CommandNames {
        &["reboot", "restart"]
    }

    fn description(&self) -> &str {
        "Reboot the system."
    }

    fn help(&self) -> &str {
        "Reboot the system.\n\
        Usage: reboot"
    }

    fn run(&self, _shell: &mut Shell, args: CommandArgs) -> Result<()> {
        if !args.is_empty() {
            bail!("Invalid usage");
        }

        if cfg!(target_os = "linux") && is_booted_natos() {
            #[cfg(target_os = "linux")]
            if unsafe { libc::reboot(libc::RB_AUTOBOOT) } != 0 {
                bail!("failed to reboot");
            }
        } else {
            let exe = std::env::current_exe()?;
            let args = std::env::args().collect::<Vec<_>>();
            nix::unistd::execvp(
                &CString::from_str(exe.to_string_lossy().as_ref())?,
                args.into_iter()
                    .map(|s| CString::new(s).ok().unwrap_or_default())
                    .collect::<Vec<_>>()
                    .as_slice(),
            )?;
        }

        Ok(())
    }
}
