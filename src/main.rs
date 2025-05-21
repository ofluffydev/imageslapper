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

use imageslapper::{geometry::rectangle::Rectangle, helpers, rendering::draw::ImageWrapper, text};
use rgb::Rgba;
use tracing::info;

const BLACK: rgb::Rgba<u8> = Rgba::new_alpha(0, 0, 0, 255);

/// Removes the testing-outputs directory if it exists.
fn remove_testing_outputs() -> std::time::Duration {
    let start = std::time::Instant::now();
    std::fs::remove_dir_all("testing-outputs").unwrap_or_else(|_| {
        info!("No testing-outputs directory to remove");
    });
    start.elapsed()
}

/// Loads an image from the specified path and returns it along with the elapsed time.
#[allow(dead_code)]
fn load_image(image_path: &str) -> (image::DynamicImage, std::time::Duration) {
    let start = std::time::Instant::now();
    let image = image::open(image_path).expect("Failed to load image");
    (image, start.elapsed())
}

/// Loads an image, using `qoi` if the extension is `.qoi`, or falls back to the default loader.
fn load_image_with_qoi(image_path: &str) -> (image::DynamicImage, std::time::Duration) {
    let start = std::time::Instant::now();
    let extension = std::path::Path::new(image_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");

    info!("Loading image: {}", image_path);
    info!("Detected extension: {}", extension);

    let image = if extension.eq_ignore_ascii_case("qoi") {
        info!("Using QOI loader for image: {}", image_path);
        image::open(image_path).expect("Failed to load QOI image")
    } else {
        let cache_path = format!("./.cache/{image_path}.qoi");
        if std::path::Path::new(&cache_path).exists() {
            info!("Loading image from cache: {}", cache_path);
            image::open(&cache_path).expect("Failed to load cached image")
        } else {
            info!("Loading image directly: {}", image_path);
            let image = image::open(image_path).expect("Failed to load image");
            let cache_dir = std::path::Path::new(&cache_path)
                .parent()
                .expect("Failed to get cache directory");
            std::fs::create_dir_all(cache_dir).expect("Failed to create cache directory");
            image
                .save(&cache_path)
                .expect("Failed to save image to cache");
            info!("Image saved to cache: {}", cache_path);
            image
        }
    };

    (image, start.elapsed())
}

/// Reads the metadata of an image and returns its size and the elapsed time.
fn read_image_metadata(image_path: &str) -> (u64, std::time::Duration) {
    let start = std::time::Instant::now();
    let size = std::fs::metadata(image_path)
        .expect("Failed to read image size")
        .len();
    info!("{size} bytes, {} kb", size / 1024);
    (size, start.elapsed())
}

/// Creates an `ImageWrapper` from `DynamicImage` and returns it along with the elapsed time.
fn create_image_wrapper(image: image::DynamicImage) -> (ImageWrapper, std::time::Duration) {
    let start = std::time::Instant::now();
    let wrapper = ImageWrapper::new(image);
    (wrapper, start.elapsed())
}

/// Draws a filled rectangle on the image and returns the elapsed time.
fn draw_rectangle(wrapper: &mut ImageWrapper, width: u32, height: u32) -> std::time::Duration {
    let start = std::time::Instant::now();
    wrapper.draw(&Rectangle::new(0, 0, width, height).filled(BLACK));
    start.elapsed()
}

/// Renders text on the image and returns the elapsed time.
fn render_text(wrapper: &mut ImageWrapper, width: u32, height: u32) -> std::time::Duration {
    let bounding_box = Rectangle::new(0, 0, width, height / 10);
    let start = std::time::Instant::now();
    let text = text::helpers::simple_open_sans("Hello, world!", bounding_box)
        .expect("Failed to create text");
    wrapper.draw(&text);
    start.elapsed()
}

/// Creates the output directory for saving images and returns the elapsed time.
fn create_output_directory() -> std::time::Duration {
    let start = std::time::Instant::now();
    std::fs::create_dir_all("testing-outputs").expect("Failed to create output directory");
    start.elapsed()
}

/// Saves the image to the specified output path and returns the elapsed time.
fn save_image(image: &image::DynamicImage, output_path: &str) -> std::time::Duration {
    let start = std::time::Instant::now();
    image.save(output_path).expect("Failed to save image");
    start.elapsed()
}

/// Placeholder for barcode demo.
fn barcode_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // Create a barcode
    let content = "123456 jump!";
    let image = helpers::code128::from(content, 500, 500, 25).expect("Failed to build code128 barcode");

    // Write the image to a file
    save_image(&image, "testing-outputs/Code128.png");
    start.elapsed()
}

/// Placeholder for coordinates demo.
fn coordinates_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for fields demo.
fn fields_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for inverted multiline demo.
fn inverted_multiline_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for QR demo.
fn qr_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for single line demo.
fn single_line_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

/// Placeholder for text anchors demo.
fn text_anchors_demo() -> std::time::Duration {
    let start = std::time::Instant::now();
    // ...logic to be implemented...
    start.elapsed()
}

#[tokio::main]
async fn main() {
    // Tracing subscriber is goated
    let filter = tracing_subscriber::EnvFilter::new("debug");
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let time_remove_dir = remove_testing_outputs();

    let image_path = "testing-inputs/image.jpg";
    // let (image, time_load_image) = load_image(image_path);
    // Use qoi instead
    let (image, time_load_image) = load_image_with_qoi(image_path);

    let (_, time_read_metadata) = read_image_metadata(image_path);

    let (mut wrapper, time_create_wrapper) = create_image_wrapper(image);

    let width = wrapper.get_image().width();
    let height = wrapper.get_image().height() / 2;
    let time_draw_rectangle = draw_rectangle(&mut wrapper, width, height);

    let time_render_text = render_text(&mut wrapper, width, height);

    let new_image = wrapper.get_image();

    let time_create_output_dir = create_output_directory();

    let time_save_image = save_image(new_image, "testing-outputs/image.png");

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
        "Creating ImageWrapper: {:?}",
        time_create_wrapper.saturating_sub(time_read_metadata)
    );
    info!(
        "Drawing rectangle: {:?}",
        time_draw_rectangle.saturating_sub(time_create_wrapper)
    );
    info!("Rendering text: {:?}", time_render_text);
    info!(
        "Creating output directory: {:?}",
        time_create_output_dir.saturating_sub(time_draw_rectangle)
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
            + time_create_wrapper
            + time_draw_rectangle
            + time_render_text
            + time_create_output_dir
            + time_save_image
    );

    info!("Demo timings:");
    info!("Barcode demo: {:?}", time_barcode_demo);
    info!("Coordinates demo: {:?}", time_coordinates_demo);
    info!("Fields demo: {:?}", time_fields_demo);
    info!("Inverted multiline demo: {:?}", time_inverted_multiline_demo);
    info!("QR demo: {:?}", time_qr_demo);
    info!("Single line demo: {:?}", time_single_line_demo);
    info!("Text anchors demo: {:?}", time_text_anchors_demo);
}
