use barcoders::sym::code128::Code128;
use barcoders::generators::image::*;
use image::{DynamicImage, RgbaImage, Rgba};

/// Generates a Code128 barcode image from the given input string, with specified dimensions and margins.
pub fn from(input: &str, width: u32, height: u32, margin: u32) -> Result<DynamicImage, String> {
    if input.is_empty() {
        return Err("Input string cannot be empty".to_string());
    }
    if !input.is_ascii() {
        return Err("Input string contains invalid characters. Only ASCII characters are allowed.".to_string());
    }

    // Determine the starting character based on the character set (This is stupid, I'll pr this crate later.)
    let starting_char = if input.chars().all(|c| c.is_ascii_alphanumeric()) {
        'À' // Character set A
    } else if input.is_ascii() {
        'Ɓ' // Character set B
    } else {
        'Ć' // Character set C
    };

    // Prepend the starting character to the input
    let formatted_input = format!("{}{}", starting_char, input);

    // Generate the barcode
    let barcode = Code128::new(&formatted_input).map_err(|e| e.to_string())?;
    let png = Image::png(height); // Barcode height in pixels
    let encoded = barcode.encode();
    let png_vec = png.generate(&encoded).map_err(|e| e.to_string())?;

    // Load the barcode image
    let barcode_image = image::load_from_memory(&png_vec).map_err(|e| e.to_string())?;
    let barcode_width = barcode_image.width();
    let barcode_height = barcode_image.height();

    // Create a blank image with the specified dimensions and a solid white background
    let mut canvas = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));

    // Resize the barcode image while maintaining its aspect ratio
    let resized_barcode = image::imageops::resize(
        &barcode_image.to_rgba8(),
        (width - margin * 2).min(barcode_width),
        (height - margin * 2).min(barcode_height),
        image::imageops::FilterType::Lanczos3,
    );

    // Recalculate offsets to center the resized barcode
    let x_offset = margin + (width - margin * 2 - resized_barcode.width()) / 2;
    let y_offset = margin + (height - margin * 2 - resized_barcode.height()) / 2;

    // Overlay the resized barcode onto the canvas
    image::imageops::overlay(&mut canvas, &resized_barcode, x_offset.into(), y_offset.into());

    Ok(DynamicImage::ImageRgba8(canvas))
}
