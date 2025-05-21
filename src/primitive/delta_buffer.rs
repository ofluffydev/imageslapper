use tracing::debug;

use crate::geometry::rectangle::Rectangle;

/// Represents a single pixel change
#[derive(Debug, Clone, Copy)]
pub struct PixelDelta {
    /// X coordinate
    pub x: u32,
    /// Y coordinate  
    pub y: u32,
    /// New RGBA pixel value
    pub color: [u8; 4],
}

/// Collection of pixel changes to be applied
#[derive(Debug, Default)]
pub struct DeltaBuffer {
    /// Vector of pixel changes
    pub changes: Vec<PixelDelta>,
    /// Optional hints about affected regions for optimizations
    pub dirty_regions: Option<Vec<Rectangle>>,
}

impl DeltaBuffer {
    pub fn log_summary(&self) {
        debug!(
            "DeltaBuffer contains {} pixel changes and {} dirty regions.",
            self.changes.len(),
            self.dirty_regions
                .as_ref()
                .map_or(0, |regions| regions.len())
        );
    }
}
