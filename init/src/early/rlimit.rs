use libc::{RLIMIT_NOFILE, RLIMIT_NPROC};

pub fn early_rlimit() -> std::io::Result<()> {
    println!("[natOS.init] configuring rlimits");
    set_process_limit(RLIMIT_NOFILE, Some(65535))?;
    set_process_limit(RLIMIT_NPROC, Some(10240))?;
    Ok(())
}

#[cfg(target_env = "gnu")]
type RLimit = libc::__rlimit_resource_t;
#[cfg(not(target_env = "gnu"))]
type RLimit = libc::c_int;

fn set_process_limit(resource: RLimit, limit: Option<u64>) -> std::io::Result<()> {
    let unpacked_limit = if let Some(rl) = limit {
        rl
    } else {
        libc::RLIM_INFINITY
    };

    let rlimit = libc::rlimit {
        rlim_cur: unpacked_limit,
        rlim_max: unpacked_limit,
    };

    unsafe {
        if libc::setrlimit(resource, &rlimit) == -1 {
            Err(std::io::Error::from_raw_os_error(libc::EINVAL))
        } else {
            Ok(())
        }
    }
}
