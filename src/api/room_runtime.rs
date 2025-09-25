//! Room Runtime adapter - Layout engine integration helpers
//!
//! This module provides specialized adapters for Room Runtime and other
//! layout engines that need detailed positioning information.
//!
//! # Features
//! - Component offset calculation
//! - Line-by-line position tracking
//! - Geometry metadata extraction
//! - Zero-copy where possible

use crate::api::layout::BoxLayout;
use crate::api::geometry::BoxDimensions;

/// Position information for a component in the layout
#[derive(Debug, Clone)]
pub struct ComponentPosition {
    /// Starting line (0-indexed)
    pub start_line: usize,
    /// Ending line (exclusive)
    pub end_line: usize,
    /// Component width in columns
    pub width: usize,
    /// Component height in rows
    pub height: usize,
    /// Component type identifier
    pub component_type: ComponentType,
}

/// Type of component in the layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Header,
    Body,
    Status,
    Footer,
}

/// Room Runtime adapter for detailed layout information
///
/// Provides component offsets and positioning data that Room Runtime
/// needs for precise layout control.
#[derive(Debug)]
pub struct RoomRuntimeAdapter {
    /// The box layout being adapted
    layout: BoxLayout,
    /// Calculated component positions
    positions: Vec<ComponentPosition>,
    /// Total height of the rendered box
    total_height: usize,
}

impl RoomRuntimeAdapter {
    /// Create a new adapter from a BoxLayout
    pub fn new(layout: BoxLayout) -> Self {
        let positions = Self::calculate_positions(&layout);
        let total_height = positions.iter()
            .map(|p| p.height)
            .sum();

        Self {
            layout,
            positions,
            total_height,
        }
    }

    /// Get the component positions
    pub fn positions(&self) -> &[ComponentPosition] {
        &self.positions
    }

    /// Get component at a specific line
    pub fn component_at_line(&self, line: usize) -> Option<(&ComponentPosition, ComponentType)> {
        self.positions.iter()
            .find(|pos| line >= pos.start_line && line < pos.end_line)
            .map(|pos| (pos, pos.component_type))
    }

    /// Get the rendered lines
    pub fn lines(&self) -> Vec<String> {
        self.layout.render_lines()
    }

    /// Get lines for a specific component
    pub fn component_lines(&self, component_type: ComponentType) -> Option<Vec<String>> {
        let position = self.positions.iter()
            .find(|p| p.component_type == component_type)?;

        let all_lines = self.lines();
        Some(all_lines[position.start_line..position.end_line].to_vec())
    }

    /// Get total height
    pub fn total_height(&self) -> usize {
        self.total_height
    }

    /// Get total width
    pub fn total_width(&self) -> usize {
        self.layout.total_width
    }

    /// Calculate component positions from layout
    fn calculate_positions(layout: &BoxLayout) -> Vec<ComponentPosition> {
        let mut positions = Vec::new();
        let mut current_line = 0;

        // Header
        if let Some(header) = &layout.header {
            let height = header.content.lines().count();
            positions.push(ComponentPosition {
                start_line: current_line,
                end_line: current_line + height,
                width: header.width,
                height,
                component_type: ComponentType::Header,
            });
            current_line += height;
        }

        // Body (always present)
        let body_height = layout.body.content.lines().count();
        positions.push(ComponentPosition {
            start_line: current_line,
            end_line: current_line + body_height,
            width: layout.body.width,
            height: body_height,
            component_type: ComponentType::Body,
        });
        current_line += body_height;

        // Status
        if let Some(status) = &layout.status {
            let height = status.content.lines().count();
            positions.push(ComponentPosition {
                start_line: current_line,
                end_line: current_line + height,
                width: status.width,
                height,
                component_type: ComponentType::Status,
            });
            current_line += height;
        }

        // Footer
        if let Some(footer) = &layout.footer {
            let height = footer.content.lines().count();
            positions.push(ComponentPosition {
                start_line: current_line,
                end_line: current_line + height,
                width: footer.width,
                height,
                component_type: ComponentType::Footer,
            });
        }

        positions
    }
}

/// Layout metadata for Room Runtime
///
/// Provides all geometry and positioning information in a single struct
#[derive(Debug, Clone)]
pub struct LayoutMetadata {
    /// Box dimensions
    pub dimensions: BoxDimensions,
    /// Component positions
    pub positions: Vec<ComponentPosition>,
    /// Has header component
    pub has_header: bool,
    /// Has footer component
    pub has_footer: bool,
    /// Has status component
    pub has_status: bool,
}

impl LayoutMetadata {
    /// Extract metadata from a BoxLayout
    pub fn from_layout(layout: &BoxLayout, dimensions: BoxDimensions) -> Self {
        let adapter = RoomRuntimeAdapter::new(layout.clone());

        Self {
            dimensions,
            positions: adapter.positions.clone(),
            has_header: layout.header.is_some(),
            has_footer: layout.footer.is_some(),
            has_status: layout.status.is_some(),
        }
    }

    /// Get content area bounds (excluding borders)
    pub fn content_bounds(&self) -> (usize, usize, usize, usize) {
        (
            1,  // start_col (after left border)
            1,  // start_row (after top border)
            self.dimensions.inner_width,
            self.dimensions.inner_height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::layout::{BoxBuilder, HeaderBuilder, FooterBuilder};

    #[test]
    fn test_adapter_positions() {
        let layout = BoxBuilder::new("Body content")
            .with_header(HeaderBuilder::new("Header"))
            .with_footer(FooterBuilder::new("Footer"))
            .build();

        let adapter = RoomRuntimeAdapter::new(layout);
        let positions = adapter.positions();

        assert_eq!(positions.len(), 3); // header, body, footer
        assert_eq!(positions[0].component_type, ComponentType::Header);
        assert_eq!(positions[1].component_type, ComponentType::Body);
        assert_eq!(positions[2].component_type, ComponentType::Footer);
    }

    #[test]
    fn test_component_at_line() {
        let layout = BoxBuilder::new("Line 1\nLine 2\nLine 3")
            .with_header(HeaderBuilder::new("Header"))
            .build();

        let adapter = RoomRuntimeAdapter::new(layout);

        // First line should be header
        let (pos, comp_type) = adapter.component_at_line(0).unwrap();
        assert_eq!(comp_type, ComponentType::Header);

        // Lines after header should be body
        let (pos, comp_type) = adapter.component_at_line(1).unwrap();
        assert_eq!(comp_type, ComponentType::Body);
    }
}