use image::DynamicImage;

use crate::{geometry::rectangle::Rectangle, rendering::draw::ImageWrapper, text::Text};

pub mod batch;

pub struct ImageBuilder {
    content: ImageWrapper,
}

impl Default for ImageBuilder {
    fn default() -> Self {
        ImageBuilder::new(1000, 1000)
    }
}

impl ImageBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        let content = ImageWrapper::new(DynamicImage::new_rgba8(width, height));
        ImageBuilder { content }
    }

    pub fn from_image(image: DynamicImage) -> Self {
        let content = ImageWrapper::new(image);
        ImageBuilder { content }
    }

    pub fn add_text(&mut self, text: Text) -> &mut Self {
        self.content.draw(&text);
        self
    }

    pub fn add_rectangle(&mut self, rectangle: Rectangle) -> &mut Self {
        self.content.draw(&rectangle);
        self
    }

    pub fn get_image(&self) -> DynamicImage {
        self.content.get_image().to_owned()
    }

    pub fn save_image(&self, path: &str) {
        self.content.save_image(path);
    }
}
