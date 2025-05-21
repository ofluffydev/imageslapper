use image::DynamicImage;
use rgb::Rgba;
use tracing::debug;

use crate::primitive::layer::{BlendMode, Layer, PixelProvider, Transform};
use crate::rendering::draw::Drawable;

/// Border types for rectangles.
#[derive(Clone, Copy, Debug)]
pub enum BorderType {
    Solid,
    Dashed,
}

/// Border for a rectangle.
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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

    /// Converts the rectangle into a layer for rendering.
    pub fn to_layer(&self) -> Layer<Self> {
        Layer {
            content: *self,
            position: (self.x1 as f64, self.y1 as f64),
            z_index: 0,                       // Default z-index
            opacity: 1.0,                     // Default opacity
            blend_mode: BlendMode::Normal,    // Default blend mode
            transform: Transform::identity(), // No transformations by default
            visible: true,                    // Default visibility
            clip_mask: None,                  // No clipping mask by default
            last_render_hash: 0,              // Default hash
        }
    }
}

impl PixelProvider for Rectangle {
    fn pixel_at(&self, x: u32, y: u32) -> rgb::Rgba<u8> {
        if self.filled {
            return self.fill_color.unwrap_or(Rgba::new(0, 0, 0, 255));
        }

        if let Some(border) = &self.border {
            let within_x_bounds = x >= self.x1 && x < self.x2;
            let within_y_bounds = y >= self.y1 && y < self.y2;

            let on_left_or_right_border = within_y_bounds
                && (x < self.x1 + border.thickness as u32
                    || x >= self.x2 - border.thickness as u32);
            let on_top_or_bottom_border = within_x_bounds
                && (y < self.y1 + border.thickness as u32
                    || y >= self.y2 - border.thickness as u32);

            if on_left_or_right_border || on_top_or_bottom_border {
                return border.color;
            }
        }

        Rgba::new(0, 0, 0, 0) // Transparent
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut DynamicImage) {
        debug!(
            "Drawing rectangle at ({}, {}) with dimensions {}x{}",
            self.x1, self.y1, self.width, self.height
        );
        let layer = self.to_layer();
        let delta_buffer = layer.collect_changes(None);

        debug!(
            "Collected {} pixel changes for rendering.",
            delta_buffer.changes.len()
        );

        // Apply the delta buffer to the image
        let mut img = image.to_rgba8();
        for change in delta_buffer.changes {
            let idx = ((change.y * img.width() + change.x) * 4) as usize;
            img.as_mut()[idx..idx + 4].copy_from_slice(&change.color);
        }

        *image = DynamicImage::ImageRgba8(img);
    }
}
