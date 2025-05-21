use rgb::Rgba;
use rusttype::Font;

use crate::geometry::rectangle::Rectangle;

use super::{Text, TextAlignment};

/// Creates a simple text object with the Open Sans font.
pub fn simple_open_sans(content: &str, bounding_box: Rectangle) -> Result<Text, String> {
    // From: https://www.1001fonts.com/open-sans-font.html "Free for commercial use"
    let font_data: &[u8] = include_bytes!("../../fonts/OpenSans-Regular.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    let color = Some(Rgba::new_alpha(255, 255, 255, 255));
    let field = Some(bounding_box);
    let alignment = TextAlignment::Left;
    let max_width = Some(100);
    let line_height = Some(1.5);
    let rotation_deg = None;
    let word_wrap = true;

    Ok(Text {
        content: content.to_string(),
        font: Some(font),
        font_size: 12.0,
        color,
        field,
        alignment,
        max_width,
        line_height,
        rotation_deg,
        word_wrap,
        anchor: None,
    })
}
