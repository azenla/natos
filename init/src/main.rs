use init::cmdline::{CMDLINE_KEY_INIT_EXECUTE, KernelCommandLine};
use init::early::early;
use nix::unistd::execv;
use std::ffi::CString;
use std::fs::{File, exists};
use std::io::{Result, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn executable_to_arg0(executable: impl AsRef<str>) -> CString {
    let path = PathBuf::from(executable.as_ref());
    path.file_name()
        .and_then(|name| CString::new(name.to_string_lossy().to_string()).ok())
        .unwrap_or_default()
}

fn run() -> Result<()> {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "unknown"
    };

    println!("[natOS.init] booted on {arch}",);

    let early = early()?;

    let cmdline = KernelCommandLine::load();

    let maybe_next = cmdline.last(CMDLINE_KEY_INIT_EXECUTE);
    let next = maybe_next.as_deref().unwrap_or("/bin/kitty");

    let mut args = shlex::split(next)
        .unwrap_or_else(|| next.split(" ").map(|item| item.to_string()).collect());

    let command = shlex::try_join(args.iter().map(|it| it.as_str())).unwrap_or_default();
    println!("[natOS.init] starting next: {command}");

    if !early.console_available {
        unsafe { std::env::set_var("CONSOLE_UNAVAILABLE", "1") };
    }

    let executable = args.remove(0);
    let arg0 = executable_to_arg0(&executable);
    let next = CString::new(executable)?;
    let mut args_c = Vec::new();
    args_c.push(arg0);
    for item in args {
        args_c.push(CString::new(item)?);
    }
    execv(&next, &args_c)?;
    Ok(())
}

fn handle_error(error: &str) {
    println!("[natOS.init] failed to launch: {error}");

    if exists("/dev/kmsg").unwrap_or(false)
        && let Ok(mut kmsg) = File::options().write(true).open("/dev/kmsg")
    {
        let _ = kmsg.write_all(format!("[natOS.init] failed to launch: {error}\n").as_bytes());
    }

    loop {
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    let panic = std::panic::catch_unwind(|| {
        if let Err(error) = run() {
            handle_error(&error.to_string());
        }
    });

    if let Err(_error) = panic {
        handle_error("full panic");
    }
}
