pub mod helpers;

use rgb::Rgba;
use rusttype::{Font, Scale};

use crate::geometry::rectangle::Rectangle;
use crate::rendering::draw::Drawable;
use image::DynamicImage;

/// Options for Text structs text alignment.
#[derive(PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// A struct representing a text object.
pub struct Text<'a> {
    pub content: String,
    pub font: Option<Font<'a>>,
    pub font_size: f32,
    pub color: Option<Rgba<u8>>,
    pub field: Option<Rectangle>,
    pub alignment: TextAlignment,
    pub max_width: Option<u32>,
    pub line_height: Option<f32>,
    pub rotation_deg: Option<f32>,
    pub word_wrap: bool,
}

/// A trait for drawable objects.
impl<'a> Drawable for Text<'a> {
    /// Draws the text on the given image.
    fn draw(&self, image: &mut DynamicImage) {
        if let Some(font) = &self.font {
            let mut scale = Scale::uniform(self.font_size);
            let color = self.color.unwrap_or(Rgba::new(0, 0, 0, 255));
            let color = image::Rgba([color.r, color.g, color.b, color.a]);

            let mut img = image.to_rgba8();
            let draw_x = self.field.map_or(0, |field| field.x1);
            let draw_y = self.field.map_or(0, |field| field.y1);

            if let Some(field) = &self.field {
                let max_width = field.width as f32;
                let mut caret = 0.0;

                while caret < max_width {
                    caret = 0.0;
                    scale = Scale::uniform(scale.x + 1.0);
                    for c in self.content.chars() {
                        if let Some(glyph) = font.glyph(c).into() {
                            caret += glyph.scaled(scale).h_metrics().advance_width;
                        }
                    }
                    if caret > max_width {
                        scale = Scale::uniform(scale.x - 1.0);
                        break;
                    }
                }
            }

            let v_metrics = font.v_metrics(scale);
            let mut glyphs = Vec::new();
            let mut caret = draw_x as f32;

            let line_width: f32 = self
                .content
                .chars()
                .filter_map(|c| {
                    std::convert::Into::<Option<rusttype::Glyph<'_>>>::into(font.glyph(c))
                        .map(|glyph| glyph.scaled(scale).h_metrics().advance_width)
                })
                .sum();

            let alignment_offset = match self.alignment {
                TextAlignment::Left => 0.0,
                TextAlignment::Center => {
                    (self.field.map_or(0, |field| field.width) as f32 - line_width) / 2.0
                }
                TextAlignment::Right => {
                    self.field.map_or(0, |field| field.width) as f32 - line_width
                }
                TextAlignment::Justify => 0.0,
            };

            caret += alignment_offset;

            for c in self.content.chars() {
                if let Some(glyph) =
                    std::convert::Into::<Option<rusttype::Glyph<'_>>>::into(font.glyph(c))
                {
                    let scaled_glyph = glyph
                        .scaled(scale)
                        .positioned(rusttype::point(caret, draw_y as f32 + v_metrics.ascent));
                    caret += scaled_glyph.unpositioned().h_metrics().advance_width;

                    if self.alignment == TextAlignment::Justify && c == ' ' {
                        let remaining_space =
                            self.field.map_or(0, |field| field.width) as f32 - line_width;
                        let space_count = self.content.matches(' ').count() as f32;
                        caret += remaining_space / space_count;
                    }

                    glyphs.push(scaled_glyph);
                }
            }

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let px = draw_x + x + bounding_box.min.x as u32;
                        let py = draw_y + y + bounding_box.min.y as u32;
                        if px < img.width() && py < img.height() {
                            let pixel = img.get_pixel_mut(px, py);
                            let bg = pixel.0;
                            let alpha = (v * color[3] as f32) as u8;

                            let blended = [
                                ((color[0] as u16 * alpha as u16
                                    + bg[0] as u16 * (255 - alpha) as u16)
                                    / 255) as u8,
                                ((color[1] as u16 * alpha as u16
                                    + bg[1] as u16 * (255 - alpha) as u16)
                                    / 255) as u8,
                                ((color[2] as u16 * alpha as u16
                                    + bg[2] as u16 * (255 - alpha) as u16)
                                    / 255) as u8,
                                bg[3],
                            ];
                            *pixel = image::Rgba(blended);
                        }
                    });
                }
            }

            *image = DynamicImage::ImageRgba8(img);
        }
    }
}
