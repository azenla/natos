use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use image::imageops::{FilterType, resize};
use image::math::Rect;

pub fn fit_to_frame(image: &DynamicImage, frame: Rect) -> Rect {
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

pub fn resize_to_display(image: &DynamicImage, frame: Rect) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let image = image.to_rgba8();
    resize(&image, frame.width, frame.height, FilterType::Lanczos3)
}

pub fn copy_to_display(output: &mut [u8], input: &[u8], output_rect: Rect, input_rect: Rect) {
    let output_stride = 4 * output_rect.width;
    let input_stride = 4 * input_rect.width;

    for row in 0..output_rect.height {
        let mut output_index: usize = (row * output_stride) as usize;
        let mut input_index: usize = ((row - input_rect.y) * input_stride) as usize;

        let is_inside_row = row >= input_rect.y && row < (input_rect.y + input_rect.height);

        for column in 0..output_rect.width {
            let is_inside_column =
                column >= input_rect.x && column < (input_rect.x + input_rect.width);

            if (input_index + 3) >= input.len() {
                continue;
            }

            if is_inside_row && is_inside_column {
                let r = input[input_index];
                let g = input[input_index + 1];
                let b = input[input_index + 2];
                let a = input[input_index + 3];
                output[output_index] = b;
                output[output_index + 1] = g;
                output[output_index + 2] = r;
                output[output_index + 3] = a;
                input_index += 4;
            }
            output_index += 4;
        }
    }
}
