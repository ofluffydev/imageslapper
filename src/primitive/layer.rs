use tracing::debug;

use crate::geometry::rectangle::Rectangle;

use super::delta_buffer::{DeltaBuffer, PixelDelta};

/// Layers for rendering on images.
#[derive(Debug, Clone)]
pub struct Layer<T: PixelProvider> {
    /// The content of this layer (generic to support different content types)
    pub content: T,
    /// Position of the layer (x, y)
    pub position: (f64, f64),
    /// Z-index for controlling stacking order
    pub z_index: i32,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque)
    pub opacity: f32,
    /// Blend mode for compositing with layers below
    pub blend_mode: BlendMode,
    /// Transformations applied to this layer
    pub transform: Transform,
    /// Whether the layer is visible
    pub visible: bool,
    /// Optional clipping mask
    pub clip_mask: Option<ClipMask>,
    /// Hash of properties that affect rendering
    pub last_render_hash: u64,
}

/// Blend modes for rendering layers.
#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
}

impl<T: PixelProvider> Layer<T> {
    /// Instead of rendering directly, collect pixel changes
    pub fn collect_changes(&self, prev_state: Option<&LayerState>) -> DeltaBuffer {
        debug!(
            "Collecting changes for layer at position ({}, {}), visible: {}",
            self.position.0, self.position.1, self.visible
        );

        let mut delta = DeltaBuffer::default();

        // Skip if layer is invisible or hasn't changed
        if !self.visible || (prev_state.is_some() && self.unchanged_since(prev_state.unwrap())) {
            debug!("Layer is either invisible or unchanged since the previous state.");
            return delta;
        }

        // Determine affected bounds (could be whole layer or just changed parts)
        let bounds = self.get_affected_bounds(prev_state);
        debug!(
            "Affected bounds: ({}, {}, {}, {})",
            bounds.x1, bounds.y1, bounds.width, bounds.height
        );

        // For each pixel in bounds that's different from previous state
        for y in bounds.y1..bounds.y1 + bounds.height {
            for x in bounds.x1..bounds.x1 + bounds.width {
                let new_color = self.compute_pixel_at(x, y);

                // If there's no previous state or the pixel changed
                if prev_state.is_none() || prev_state.unwrap().pixel_at(x, y) != new_color {
                    delta.changes.push(PixelDelta {
                        x,
                        y,
                        color: new_color.into(),
                    });
                }
            }
        }

        debug!(
            "Collected {} pixel changes for the layer.",
            delta.changes.len()
        );

        // Add bounds as dirty region for potential optimizations
        if !delta.changes.is_empty() {
            delta
                .dirty_regions
                .get_or_insert_with(Vec::new)
                .push(bounds);
        }

        delta
    }

    /// Checks if the layer has remained unchanged since the previous state.
    pub fn unchanged_since(&self, prev_state: &LayerState) -> bool {
        self.last_render_hash == prev_state.hash
    }

    /// Computes the color of a pixel at the given coordinates (x, y) for this layer.
    pub fn compute_pixel_at(&self, x: u32, y: u32) -> rgb::Rgba<u8> {
        // Apply transformations to determine the source coordinates
        let transformed_x = ((x as f64 - self.position.0) / self.transform.scale_x) as u32;
        let transformed_y = ((y as f64 - self.position.1) / self.transform.scale_y) as u32;

        // Check if the transformed coordinates are within the content bounds
        if transformed_x >= self.width() || transformed_y >= self.height() {
            return rgb::Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }; // Transparent pixel
        }

        // Retrieve the base color from the content (assuming content implements a `pixel_at` method)
        let base_color = self.content.pixel_at(transformed_x, transformed_y);

        // Apply opacity to the base color
        let alpha = (base_color.a as f32 * self.opacity) as u8;

        // Return the final color with adjusted alpha
        rgb::Rgba {
            r: base_color.r,
            g: base_color.g,
            b: base_color.b,
            a: alpha,
        }
    }

    /// Determines the bounds affected by the layer, considering its current and previous state.
    pub fn get_affected_bounds(&self, prev_state: Option<&LayerState>) -> Rectangle {
        if let Some(prev) = prev_state {
            // Combine current and previous bounds to account for changes.
            let x1 = self.position.0.min(prev.x as f64) as u32;
            let y1 = self.position.1.min(prev.y as f64) as u32;
            let width = (self.transform.scale_x * self.width() as f64) as u32;
            let height = (self.transform.scale_y * self.height() as f64) as u32;
            Rectangle {
                x1,
                y1,
                width: (self.position.0.max(prev.x as f64)
                    + self.transform.scale_x * self.width() as f64) as u32,
                height: (self.position.1.max(prev.y as f64)
                    + self.transform.scale_y * self.height() as f64) as u32,
                x2: x1 + width,
                y2: y1 + height,
                border: None,
                filled: false,
                fill_color: None,
            }
        } else {
            // Use the current layer's bounds if no previous state exists.
            let x1 = self.position.0 as u32;
            let y1 = self.position.1 as u32;
            let width = (self.transform.scale_x * self.width() as f64) as u32;
            let height = (self.transform.scale_y * self.height() as f64) as u32;
            Rectangle {
                x1,
                y1,
                width,
                height,
                x2: x1 + width,
                y2: y1 + height,
                border: None,
                filled: false,
                fill_color: None,
            }
        }
    }

    /// Helper method to get the width of the layer's content.
    fn width(&self) -> u32 {
        self.content.width()
    }

    /// Helper method to get the height of the layer's content.
    fn height(&self) -> u32 {
        self.content.height()
    }
}

/// Represents the state of a layer for comparison purposes.
pub struct LayerState {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: rgb::Rgba<u8>,
    pub opacity: f32,
    pub z_index: i32,
    pub blend_mode: BlendMode,
    /// Hash of properties that affect rendering
    pub hash: u64,
}

impl LayerState {
    /// Get the color of a pixel at the given coordinates.
    pub fn pixel_at(&self, x: u32, y: u32) -> rgb::Rgba<u8> {
        if x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height {
            self.color
        } else {
            rgb::Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            } // Transparent pixel if out of bounds
        }
    }
}

/// Represents an optional clipping mask for a layer.
#[derive(Debug, Clone)]
pub struct ClipMask {
    /// The bounds of the clipping mask.
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    /// The mask data (e.g., alpha values for each pixel).
    pub mask_data: Vec<u8>,
}

impl ClipMask {
    /// Checks if a given point (x, y) is within the clipping mask.
    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// Retrieves the mask value at a given point (x, y).
    pub fn value_at(&self, x: u32, y: u32) -> Option<u8> {
        if self.contains(x, y) {
            let index = ((y - self.y) * self.width + (x - self.x)) as usize;
            Some(self.mask_data[index])
        } else {
            None
        }
    }
}

/// Represents transformations applied to a layer.
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// Horizontal translation (in pixels).
    pub translate_x: f64,
    /// Vertical translation (in pixels).
    pub translate_y: f64,
    /// Scaling factor along the x-axis.
    pub scale_x: f64,
    /// Scaling factor along the y-axis.
    pub scale_y: f64,
    /// Rotation angle (in degrees).
    pub rotation: f64,
}

impl Transform {
    /// Creates a new identity transform (no transformation).
    pub fn identity() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
        }
    }
}

/// A trait that defines the `pixel_at` method for content types.
pub trait PixelProvider {
    /// Retrieves the color of a pixel at the given coordinates (x, y).
    fn pixel_at(&self, x: u32, y: u32) -> rgb::Rgba<u8>;

    /// Retrieves the width of the content.
    fn width(&self) -> u32;

    /// Retrieves the height of the content.
    fn height(&self) -> u32;
}
