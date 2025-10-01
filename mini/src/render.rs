use anyhow::Result;
use image::imageops::{FilterType, resize};
use image::math::Rect;
use image::{DynamicImage, ImageBuffer, Rgba};
use uefi::boot;
use uefi::boot::ScopedProtocol;
use uefi::proto::console::gop::{BltOp, BltRegion};
use uefi::proto::console::gop::{BltPixel, GraphicsOutput};

struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    fn blit(&self, gop: &mut GraphicsOutput) -> Result<()> {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })?;
        Ok(())
    }
}

fn setup_graphics() -> Result<ScopedProtocol<GraphicsOutput>> {
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;
    Ok(gop)
}

fn fit_to_frame(image: &DynamicImage, frame: Rect) -> Rect {
    let input = Rect {
        x: 0,
        y: 0,
        width: image.width(),
        height: image.height(),
    };

    let input_ratio = input.width as f32 / input.height as f32;
    let frame_ratio = frame.width as f32 / frame.height as f32;

    let mut output = Rect {
        x: 0,
        y: 0,
        width: frame.width,
        height: frame.height,
    };

    if input_ratio < frame_ratio {
        output.width = (frame.height as f32 * input_ratio).floor() as u32;
        output.height = frame.height;
        output.x = frame.x + (frame.width - output.width) / 2;
        output.y = frame.y;
    } else {
        output.width = frame.width;
        output.height = (frame.width as f32 / input_ratio).floor() as u32;
        output.x = frame.x;
        output.y = frame.y + (frame.height - output.height) / 2;
    }

    output
}

fn resize_to_fit(image: &DynamicImage, frame: Rect) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let image = image.to_rgba8();
    resize(&image, frame.width, frame.height, FilterType::Lanczos3)
}

pub fn draw(image: DynamicImage) -> Result<()> {
    let mut gop = setup_graphics()?;
    let (width, height) = gop.current_mode_info().resolution();
    let display_frame = Rect {
        x: 0,
        y: 0,
        width: width as _,
        height: height as _,
    };
    let fit = fit_to_frame(&image, display_frame);
    let image = resize_to_fit(&image, fit);

    let mut framebuffer = Framebuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels() {
        let Some(fb) = framebuffer.pixel((x + fit.x) as usize, (fit.y + y) as usize) else {
            continue;
        };
        fb.red = pixel[0];
        fb.green = pixel[1];
        fb.blue = pixel[2];
    }

    framebuffer.blit(&mut gop)?;
    Ok(())
}
