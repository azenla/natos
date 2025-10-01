mod rlimit;
mod sysctl;
mod util;

use crate::early::rlimit::early_rlimit;
use crate::early::sysctl::early_sysctl;
use crate::early::util::{
    copy_bin_from_initrd, create_dir, mount_kernel_fs, move_actual_to_root, nuke_initrd,
    setup_console,
};
use nix::unistd::setsid;
use std::io::Result;
use std::os::unix::fs::{chroot, symlink};
use sys_mount::{FilesystemType, Mount, MountFlags};

pub struct EarlyInitialization {
    pub console_available: bool,
}

pub fn early() -> Result<EarlyInitialization> {
    println!("[natOS.init] starting early setup");
    println!("[natOS.init] creating session");
    setsid()?;

    println!("[natOS.init] jettisoning initramfs");
    create_dir("/actual", None)?;
    Mount::builder()
        .fstype(FilesystemType::Manual("tmpfs"))
        .mount("tmpfs", "/actual")?;
    copy_bin_from_initrd()?;
    nuke_initrd()?;
    std::env::set_current_dir("/actual")?;
    move_actual_to_root()?;
    chroot(".")?;
    std::env::set_current_dir("/")?;

    println!("[natOS.init] setting up filesystem");
    create_dir("/dev", Some(0o0755))?;
    create_dir("/proc", None)?;
    create_dir("/sys", Some(0o0555))?;
    create_dir("/root", Some(0o0700))?;
    create_dir("/tmp", None)?;
    create_dir("/run", Some(0o0755))?;
    mount_kernel_fs("devtmpfs", "/dev", "mode=0755", None, None)?;
    mount_kernel_fs("proc", "/proc", "hidepid=1", None, None)?;
    mount_kernel_fs("sysfs", "/sys", "", None, None)?;
    create_dir("/dev/pts", Some(0o0755))?;
    mount_kernel_fs("devpts", "/dev/pts", "", None, Some("/dev/ptmx"))?;
    symlink("/proc/self/fd", "/dev/fd")?;
    symlink("/proc/self/fd/0", "/dev/stdin")?;
    symlink("/proc/self/fd/1", "/dev/stdout")?;
    symlink("/proc/self/fd/2", "/dev/stderr")?;
    mount_kernel_fs(
        "cgroup2",
        "/sys/fs/cgroup",
        "",
        Some(MountFlags::RELATIME),
        None,
    )?;
    println!("[natOS.init] configuring console");
    let console_available = setup_console()?;
    early_sysctl()?;
    early_rlimit()?;
    println!("[natOS.init] early setup complete");
    Ok(EarlyInitialization { console_available })
}
