use image::DynamicImage;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rgb::Rgba;

use crate::rendering::draw::Drawable;

/// Border types for rectangles.
#[derive(Clone, Copy)]
pub enum BorderType {
    Solid,
    Dashed,
}

/// Border for a rectangle.
#[derive(Clone, Copy)]
pub struct Border {
    pub thickness: u8,
    pub color: Rgba<u8>,
    pub r#type: BorderType,
}

/// Default border values.
impl Default for Border {
    fn default() -> Self {
        Border {
            thickness: 10,
            color: Rgba::new(100, 100, 100, 100),
            r#type: BorderType::Solid,
        }
    }
}

/// Rectangle struct for drawing rectangles on images.
#[derive(Clone, Copy)]
pub struct Rectangle {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
    pub width: u32,
    pub height: u32,
    pub border: Option<Border>,
    pub filled: bool,
    pub fill_color: Option<Rgba<u8>>,
}

impl Rectangle {
    pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Rectangle {
        Rectangle {
            x1,
            y1,
            x2,
            y2,
            border: Some(Border::default()),
            filled: true,
            width: x2 - x1,
            height: y2 - y1,
            fill_color: None,
        }
    }

    pub fn from_xywh(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x1: x,
            y1: y,
            x2: x + width,
            y2: y + height,
            border: Some(Border::default()),
            filled: true,
            width,
            height,
            fill_color: None,
        }
    }

    pub fn filled(&self, fill_color: Rgba<u8>) -> Self {
        let border = self.border.unwrap_or_default();
        Rectangle {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            border: Some(border),
            filled: true,
            width: self.width,
            height: self.height,
            fill_color: Some(fill_color),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut DynamicImage) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        let (width, height) = (image.width(), image.height());
        let mut img = image.to_rgba8().into_raw();

        let fill_color = self.fill_color.unwrap_or(Rgba::new(0, 0, 0, 255));
        let fill_color = image::Rgba([fill_color.r, fill_color.g, fill_color.b, fill_color.a]);

        let start_x = self.x1;
        let end_x = (self.x1 + self.width).min(width);
        let start_y = self.y1;
        let end_y = (self.y1 + self.height).min(height);

        img.par_chunks_mut((width * 4) as usize)
            .enumerate()
            .for_each(|(j, row)| {
                if j >= start_y as usize && j < end_y as usize {
                    let row_start = (start_x * 4) as usize;
                    let row_end = (end_x * 4) as usize;

                    row[row_start..row_end]
                        .chunks_exact_mut(4)
                        .for_each(|pixel| {
                            pixel.copy_from_slice(&fill_color.0);
                        });
                }
            });

        let new_image = image::ImageBuffer::from_raw(width, height, img).unwrap();
        *image = DynamicImage::ImageRgba8(new_image);
    }
}
