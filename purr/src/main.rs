use anyhow::{Result, anyhow};
use image::{DynamicImage, ImageReader};
use purr::assets::IMAGES;
use purr::gfx::display::show;
use std::env::args;
use std::io::{Cursor, Read, stdin};

fn load(name: &str) -> Result<DynamicImage> {
    let (_, bytes) = IMAGES
        .iter()
        .find(|&&(n, _)| n == name)
        .ok_or_else(|| anyhow!("unknown image: {name}"))?;

    let rgba = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?
        .into_rgba8();
    Ok(DynamicImage::ImageRgba8(rgba))
}

fn main() -> Result<()> {
    let mut items = args().skip(1).collect::<Vec<_>>();
    if items.is_empty() {
        items.extend_from_slice(&[
            "nat1".to_string(),
            "nat2".to_string(),
            "nat3".to_string(),
            "nat4".to_string(),
        ]);
    }

    if items[0] == "list" {
        for &(name, _) in IMAGES {
            println!("image: {name}");
        }
        return Ok(());
    }

    let mut images = Vec::new();
    for item in items {
        let image = load(&item)?;
        images.push(image);
    }

    show(
        &images,
        || {
            println!("Please switch over to the GPU.");
            Ok(())
        },
        || {
            println!("Press enter to view the next image.");
            let mut stdin_buffer = vec![0u8; 1];
            stdin().read_exact(&mut stdin_buffer)?;
            Ok(())
        },
        || {
            println!("Press enter to exit.");
            let mut stdin_buffer = vec![0u8; 1];
            stdin().read_exact(&mut stdin_buffer)?;
            Ok(())
        },
    )
    .map_err(|e| anyhow!("failed to show image: {e}"))?;
    Ok(())
}
