//! Visual helpers - RSB MODULE_SPEC internal implementation
//!
//! This module contains internal helper functions for the visual system.
//! These are implementation details that support the public API in utils.rs.
//!
//! Consolidated from boxes.rs, draw.rs, and components.rs following RSB MODULE_SPEC.
//!
//! Version: boxy v0.16.0+ (RSB MODULE_SPEC reorganization)

/// PARALLEL SOLUTION: Calculate proper inner content target width
/// This helper function determines the correct width for padding content lines
/// without breaking the existing width calculation logic
#[allow(dead_code)]
pub fn calculate_inner_content_target_width(
    inner_width: usize,
    _composed_lines: &[String],
    _is_fixed_width: bool,
    h_padding: usize,
) -> usize {
    // The target width for content should be the inner width minus padding on both sides
    inner_width.saturating_sub(2 * h_padding)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_inner_content_target_width() {
        // Test basic width calculation
        assert_eq!(calculate_inner_content_target_width(80, &[], false, 2), 76);
        assert_eq!(calculate_inner_content_target_width(40, &[], true, 1), 38);

        // Test edge case with very small width
        assert_eq!(calculate_inner_content_target_width(2, &[], false, 1), 0);
        assert_eq!(calculate_inner_content_target_width(1, &[], false, 2), 0); // saturating_sub prevents underflow
    }
}
