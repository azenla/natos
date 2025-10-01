use anyhow::Result;
use kitty::shell::commands::all::all_commands;
use kitty::shell::frontend::Shell;
use kitty::startup::startup;
use kitty::utils::pause_forever;
use std::fs::exists;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn shell() -> Result<()> {
    let mut shell = Shell::new(all_commands())?;
    shell.run()?;
    Ok(())
}

fn fallback_loop_forever() -> Result<()> {
    println!("[natOS.kitty] console and drm unavailable, sleeping forever");
    loop {
        sleep(Duration::from_secs(1));
    }
}

fn fallback_purr() -> Result<()> {
    Command::new("/bin/purr").arg("default").status()?;
    Ok(())
}

fn run() -> Result<()> {
    startup()?;

    if std::env::var("CONSOLE_UNAVAILABLE")
        .ok()
        .unwrap_or_default()
        == "1"
    {
        if exists("/dev/dri/card0")? {
            let _ = fallback_purr();
            fallback_loop_forever()?;
        }
        fallback_loop_forever()?;
    } else {
        shell()?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("[natOS.kitty] error: {e:?}");
    }

    pause_forever();
}
