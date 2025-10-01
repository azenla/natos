use anyhow::Result;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::io::Cursor;

const NAT_PNG: &[u8] = include_bytes!("nat.png");

pub fn load_image() -> Result<DynamicImage> {
    Ok(ImageReader::with_format(Cursor::new(NAT_PNG), ImageFormat::Png).decode()?)
}
