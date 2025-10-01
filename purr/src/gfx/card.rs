use drm::Device as BasicDevice;
use drm::control::Device as ControlDevice;
use std::fs::{File, OpenOptions, exists};
use std::os::fd::{AsFd, BorrowedFd};

const PRIMARY_CARD: &str = "/dev/dri/card0";

pub struct Card {
    file: File,
}

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.file.as_fd()
    }
}

impl Card {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let mut options = OpenOptions::new();
        options.read(true);
        options.write(true);
        Ok(Card {
            file: options.open(path)?,
        })
    }

    pub fn has_primary() -> anyhow::Result<bool> {
        Ok(exists(PRIMARY_CARD)?)
    }

    pub fn open_primary() -> anyhow::Result<Self> {
        Self::open(PRIMARY_CARD)
    }
}

impl BasicDevice for Card {}
impl ControlDevice for Card {}
