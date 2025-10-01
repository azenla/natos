use anyhow::Result;
use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;

fn flush_stdout() -> Result<()> {
    stdout().flush()?;
    Ok(())
}

pub fn startup() -> Result<()> {
    println!("[natOS.kitty] welcome to kitty!");
    let mut count = 5;
    print!("[natOS.kitty] launching");
    flush_stdout()?;
    loop {
        sleep(Duration::from_millis(500));
        print!(".");
        flush_stdout()?;
        count -= 1;
        if count <= 0 {
            break;
        }
    }
    println!();
    flush_stdout()?;
    Ok(())
}
