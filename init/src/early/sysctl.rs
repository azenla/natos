use std::fs::{exists, write};
use std::path::PathBuf;

fn sysctl(key: &str, value: &str) -> std::io::Result<()> {
    let mut path = PathBuf::from("/proc/sys");
    for part in key.split('.') {
        path = path.join(part);
    }

    if exists(&path)? {
        write(&path, value)?;
    }
    Ok(())
}

pub fn early_sysctl() -> std::io::Result<()> {
    println!("[natOS.init] configuring sysctls");
    sysctl("vm.max_map_count", "1048576")?;
    sysctl("kernel.pid_max", "4194304")?;
    sysctl("fs.aio-max-nr", "1048576")?;
    sysctl("net.ipv4.ip_unprivileged_port_start", "10")?;
    Ok(())
}
