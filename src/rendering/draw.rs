use std::time::{Duration, Instant};

use image::DynamicImage;

/// A trait that defines a drawable object.
pub trait Drawable {
    fn draw(&self, image: &mut DynamicImage);
}

/// A struct that wraps around an image and provides drawing functionality.
pub struct ImageWrapper {
    image: DynamicImage,
}

/// A wrapper around the image to provide drawing functionality.
impl ImageWrapper {
    /// Creates a new ImageWrapper with the given image.
    pub fn new(image: DynamicImage) -> Self {
        ImageWrapper { image }
    }

    /// Draws the given drawable on the image.
    pub fn draw<T: Drawable>(&mut self, drawable: &T) {
        drawable.draw(&mut self.image);
    }

    /// Returns a reference to the image.
    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }

    /// Saves the image to the specified output path and returns the elapsed time.
    pub fn save_image(&self, output_path: &str) -> Duration {
        let start = Instant::now();
        self.image.save(output_path).expect("Failed to save image");
        start.elapsed()
    }
}
