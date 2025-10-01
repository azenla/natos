use crate::shell::command::ShellCommand;
use crate::shell::commands::help::HelpCommand;
use crate::shell::commands::meow::MeowCommand;
use crate::shell::commands::panic::PanicCommand;
use crate::shell::commands::purrview::PurrCommand;
use crate::shell::commands::reboot::RebootCommand;
use crate::shell::commands::shutdown::ShutdownCommand;

pub fn all_commands() -> Vec<Box<dyn ShellCommand>> {
    vec![
        Box::new(HelpCommand),
        Box::new(MeowCommand),
        Box::new(PanicCommand),
        Box::new(PurrCommand),
        Box::new(RebootCommand),
        Box::new(ShutdownCommand),
    ]
}
