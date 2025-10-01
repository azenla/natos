use libc::mount;
use nix::unistd::{dup2_stderr, dup2_stdin, dup2_stdout};
use std::ffi::CString;
use std::fs;
use std::fs::{File, OpenOptions, Permissions, exists};
use std::io::{Error, Result};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::ptr::null_mut;
use sys_mount::{FilesystemType, Mount, MountFlags};
use walkdir::WalkDir;

const DEFAULT_CONSOLE_PATH: &str = "/dev/console";

pub fn create_dir(path: &str, mode: Option<u32>) -> Result<()> {
    let path = Path::new(path);
    if !path.is_dir() {
        fs::create_dir(path)?;
    }
    if let Some(mode) = mode {
        let permissions = Permissions::from_mode(mode);
        fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

pub fn mount_kernel_fs(
    fstype: &str,
    path: &str,
    data: &str,
    flags: Option<MountFlags>,
    source: Option<&str>,
) -> Result<()> {
    Mount::builder()
        .fstype(FilesystemType::Manual(fstype))
        .flags(MountFlags::NOEXEC | MountFlags::NOSUID | flags.unwrap_or(MountFlags::empty()))
        .data(data)
        .mount(source.unwrap_or(fstype), path)?;
    Ok(())
}

pub fn copy_bin_from_initrd() -> Result<()> {
    if !fs::exists("/bin")? {
        return Ok(());
    }
    create_dir("/actual/bin", None)?;
    let mut directory = fs::read_dir("/bin")?;

    loop {
        let Some(entry) = directory.next() else {
            break;
        };
        let entry = entry?;

        if !entry.file_type()?.is_file() {
            continue;
        }

        let path = entry.path();
        let Some(name) = path.file_name() else {
            continue;
        };
        let name = name.to_string_lossy().to_string();
        let final_path = format!("/actual/bin/{name}");
        fs::copy(entry.path(), final_path)?;
    }
    Ok(())
}

pub fn nuke_initrd() -> Result<()> {
    let initrd_dev = fs::metadata("/")?.st_dev();
    for item in WalkDir::new("/")
        .same_file_system(true)
        .follow_links(false)
        .contents_first(true)
    {
        if item.is_err() {
            continue;
        }

        let item = item?;

        if item.path() == Path::new("/") {
            continue;
        }

        let metadata = match item.metadata() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if metadata.st_dev() != initrd_dev {
            continue;
        }

        if metadata.is_symlink() || metadata.is_file() {
            let _ = fs::remove_file(item.path());
        } else if metadata.is_dir() {
            let _ = fs::remove_dir(item.path());
        }
    }
    Ok(())
}

pub fn move_actual_to_root() -> Result<()> {
    let dot = CString::new(".")?;
    let slash = CString::new("/")?;
    let error = unsafe {
        mount(
            dot.into_raw(),
            slash.into_raw(),
            null_mut(),
            MountFlags::MOVE.bits(),
            null_mut(),
        )
    };
    if error < 0 {
        return Err(Error::from_raw_os_error(error));
    }
    Ok(())
}

pub fn has_console() -> bool {
    exists(DEFAULT_CONSOLE_PATH).unwrap_or(false)
}

pub fn map_console(console: &File) -> Result<()> {
    dup2_stdin(console)?;
    dup2_stdout(console)?;
    dup2_stderr(console)?;
    Ok(())
}

pub fn setup_console() -> Result<bool> {
    let has_console = has_console();
    if !has_console {
        return Ok(false);
    }
    let Some(console) = OpenOptions::new()
        .read(true)
        .write(true)
        .open(DEFAULT_CONSOLE_PATH)
        .ok()
    else {
        return Ok(false);
    };
    if map_console(&console).is_err() {
        return Ok(false);
    }

    Ok(has_console)
}
