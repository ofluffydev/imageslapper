// Be a perfectionist, no code is good enough!
#![deny(
    clippy::all,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
// Unwraps are a bad practice and do not provide useful error messages/handling.
#![warn(clippy::unwrap_used)]
// This lint happens regardless and is out of our control.
#![allow(clippy::multiple_crate_versions)]

use std::time::{Duration, Instant};

use image::DynamicImage;
use imageslapper::{
    builders::ImageBuilder, geometry::rectangle::Rectangle, helpers,
    io::images::load_image_with_qoi, rendering::draw::ImageWrapper, text,
};
use tracing::info;

/// Removes the testing-outputs directory if it exists.
fn remove_testing_outputs() -> Duration {
    let start = Instant::now();
    std::fs::remove_dir_all("testing-outputs").unwrap_or_else(|_| {
        info!("No testing-outputs directory to remove");
    });
    start.elapsed()
}

/// Reads the metadata of an image and returns its size and the elapsed time.
fn read_image_metadata(image_path: &str) -> (u64, Duration) {
    let start = Instant::now();
    let size = std::fs::metadata(image_path)
        .expect("Failed to read image size")
        .len();
    info!("{size} bytes, {} kb", size / 1024);
    (size, start.elapsed())
}

/// Creates the output directory for saving images and returns the elapsed time.
fn create_output_directory() -> Duration {
    let start = Instant::now();
    std::fs::create_dir_all("testing-outputs").expect("Failed to create output directory");
    start.elapsed()
}

/// Creates an `ImageWrapper` from `DynamicImage` and returns it along with the elapsed time.
#[must_use]
pub fn create_image_wrapper(image: DynamicImage) -> (ImageWrapper, std::time::Duration) {
    let start = Instant::now();
    let wrapper = ImageWrapper::new(image);
    (wrapper, start.elapsed())
}

/// Placeholder for barcode demo.
fn barcode_demo() -> Duration {
    let start = Instant::now();

    // Create a barcode
    let content = "123456 jump!";
    let image =
        helpers::code128::from(content, 500, 500, 25).expect("Failed to build code128 barcode");

    // Write the image to a file
    ImageWrapper::new(image).save_image("testing-outputs/Code128.png");
    start.elapsed()
}

/// Placeholder for coordinates demo.
fn coordinates_demo() -> Duration {
    let start = Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for fields demo.
fn fields_demo() -> Duration {
    let start = Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for inverted multiline demo.
fn inverted_multiline_demo() -> Duration {
    let start = Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for QR demo.
fn qr_demo() -> Duration {
    let start = Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for single line demo.
fn single_line_demo() -> Duration {
    let start = Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for text anchors demo.
fn text_anchors_demo() -> Duration {
    let start = Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // Tracing subscriber is goated
    let filter = tracing_subscriber::EnvFilter::new("debug");
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let time_remove_dir = remove_testing_outputs();

    let image_path = "testing-inputs/image.avif";

    // Use qoi
    let (image, time_load_image) = load_image_with_qoi(image_path);

    let (_, time_read_metadata) = read_image_metadata(image_path);

    let rectangle_height = image.height() * 4 / 10;
    let rectangle_width = image.width();
    let rectangle = Rectangle::new(0, 0, rectangle_width, rectangle_height);
    let image = ImageBuilder::from_image(image)
        .add_rectangle(rectangle)
        .add_text(
            text::helpers::simple_open_sans("Hello, world!", rectangle)
                .expect("Failed to create text"),
        )
        .get_image();
    let time_render_text_and_rectangle = start.elapsed();

    let time_create_output_dir = create_output_directory();

    ImageWrapper::new(image).save_image("testing-outputs/image.png");
    let time_save_image = start.elapsed() - time_create_output_dir;

    let time_barcode_demo = barcode_demo();
    let time_coordinates_demo = coordinates_demo();
    let time_fields_demo = fields_demo();
    let time_inverted_multiline_demo = inverted_multiline_demo();
    let time_qr_demo = qr_demo();
    let time_single_line_demo = single_line_demo();
    let time_text_anchors_demo = text_anchors_demo();

    info!("Summary of elapsed times:");
    info!("Removing directory: {:?}", time_remove_dir);
    info!(
        "Loading image: {:?}",
        time_load_image.saturating_sub(time_remove_dir)
    );
    info!(
        "Reading metadata: {:?}",
        time_read_metadata.saturating_sub(time_load_image)
    );
    info!(
        "Rendering text and rectangle: {:?}",
        time_render_text_and_rectangle.saturating_sub(time_read_metadata)
    );
    info!(
        "Saving image: {:?}",
        time_save_image.saturating_sub(time_create_output_dir)
    );
    info!(
        "Total time elapsed: {:?}",
        time_remove_dir
            + time_load_image
            + time_read_metadata
            + time_render_text_and_rectangle
            + time_create_output_dir
            + time_save_image
    );

    info!("Demo timings:");
    info!("Barcode demo: {:?}", time_barcode_demo);
    info!("Coordinates demo: {:?}", time_coordinates_demo);
    info!("Fields demo: {:?}", time_fields_demo);
    info!(
        "Inverted multiline demo: {:?}",
        time_inverted_multiline_demo
    );
    info!("QR demo: {:?}", time_qr_demo);
    info!("Single line demo: {:?}", time_single_line_demo);
    info!("Text anchors demo: {:?}", time_text_anchors_demo);
}
