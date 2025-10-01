#![feature(uefi_std)]

pub mod image;
pub mod init;
pub mod render;

use crate::image::load_image;
use anyhow::Result;
use std::thread::sleep;
use std::time::Duration;

const WAIT_TIME: Duration = Duration::from_secs(60);

fn main() -> Result<()> {
    init::init();

    println!("Welcome to uNatOS!");
    let image = load_image()?;
    render::draw(image)?;

    sleep(WAIT_TIME);

    Ok(())
}
