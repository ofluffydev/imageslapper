use image::DynamicImage;

pub trait Drawable {
    fn draw(&self, image: &mut DynamicImage);
}

pub struct ImageWrapper {
    image: DynamicImage,
}

impl ImageWrapper {
    pub fn new(image: DynamicImage) -> Self {
        ImageWrapper { image }
    }

    pub fn draw<T: Drawable>(&mut self, drawable: &T) {
        drawable.draw(&mut self.image);
    }

    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }
}
