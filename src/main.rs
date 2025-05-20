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

use imageslapper::{geometry::rectangle::Rectangle, rendering::draw::ImageWrapper, text};
use rgb::Rgba;
use tracing::info;

const BLACK: rgb::Rgba<u8> = Rgba::new_alpha(0, 0, 0, 255);

#[tokio::main]
async fn main() {
    // Tracing subscriber is goated
    let filter = tracing_subscriber::EnvFilter::new("info");
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Start the timer
    let start = std::time::Instant::now();

    // Remove the testing-outputs directory if it exists
    std::fs::remove_dir_all("testing-outputs")
        .expect("Failed to remove directory, it may not exist");
    let time_remove_dir = start.elapsed();
    info!("Time after removing directory: {:?}", time_remove_dir);

    // Load the image
    let image_path = "testing-inputs/image.jpg";
    let image = image::open("testing-inputs/image.jpg").expect("Failed to load image");
    let time_load_image = start.elapsed();
    info!("Time after loading image: {:?}", time_load_image);

    // Read the image metadata
    let size = std::fs::metadata(image_path)
        .expect("Failed to read image size")
        .len();
    info!("{size} bytes, {} kb", size / 1024);
    let time_read_metadata = start.elapsed();
    info!(
        "Time after reading image metadata: {:?}",
        time_read_metadata
    );

    // Create an ImageWrapper (holds all our drawing functions)
    let mut wrapper = ImageWrapper::new(image);
    let time_create_wrapper = start.elapsed();
    info!(
        "Time after creating ImageWrapper: {:?}",
        time_create_wrapper
    );

    // Draw a rectangle on the image
    let width = wrapper.get_image().width();
    let height = wrapper.get_image().height() / 2;
    wrapper.draw(&Rectangle::new(0, 0, width, height).filled(BLACK));
    let time_draw_rectangle = start.elapsed();
    info!("Time after drawing rectangle: {:?}", time_draw_rectangle);

    //
    let bounding_box = Rectangle::new(0, 0, width, height / 10);
    let text_start = std::time::Instant::now();
    let text = text::helpers::simple_open_sans("Hello, world!", bounding_box)
        .expect("Failed to create text");
    let time_render_text = text_start.elapsed();
    info!("Time after rendering text: {:?}", time_render_text);
    wrapper.draw(&text);

    // Get our resulting image
    let new_image = wrapper.get_image();

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all("testing-outputs").expect("Failed to create output directory");
    let time_create_output_dir = start.elapsed();
    info!(
        "Time after creating output directory: {:?}",
        time_create_output_dir
    );

    // Save the image
    new_image
        .save("testing-outputs/image.png")
        .expect("Failed to save image");
    let time_save_image = start.elapsed();
    info!("Time after saving image: {:?}", time_save_image);

    info!("Summary of elapsed times:");
    info!("Removing directory: {:?}", time_remove_dir);
    info!("Loading image: {:?}", time_load_image - time_remove_dir);
    info!(
        "Reading metadata: {:?}",
        time_read_metadata - time_load_image
    );
    info!(
        "Creating ImageWrapper: {:?}",
        time_create_wrapper - time_read_metadata
    );
    info!(
        "Drawing rectangle: {:?}",
        time_draw_rectangle - time_create_wrapper
    );
    info!("Rendering text: {:?}", time_render_text);
    info!(
        "Creating output directory: {:?}",
        time_create_output_dir - time_draw_rectangle
    );
    info!(
        "Saving image: {:?}",
        time_save_image - time_create_output_dir
    );
    info!("Total time elapsed: {:?}", time_save_image);
}
