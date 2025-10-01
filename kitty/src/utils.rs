use std::thread::sleep;
use std::time::Duration;

pub fn pause_forever() {
    loop {
        sleep(Duration::from_secs(1));
    }
}

pub fn is_booted_natos() -> bool {
    !(std::fs::exists("/nix").unwrap_or(false) || std::fs::exists("/etc").unwrap_or(false))
}
