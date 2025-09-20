//! Status calculation macros - centralized to prevent accidental regressions.
//!
//! These macros encapsulate the exact padding/divider/rendering math that keeps
//! the status bar aligned with the rest of the box. They intentionally mirror
//! the proven logic from the last-working build so any future edits happen in a
//! single protected location.

/// PROTECTED: Render a blank padding line around the status bar.
#[macro_export]
macro_rules! status_padding_line {
    ($config:expr, $inner_width:expr, $color_code:expr) => {{
        let available_content_width = $inner_width.saturating_sub(2 * $config.width.h_padding);
        let pad = " ".repeat($config.width.h_padding);
        format!(
            "{}{}{}{}{}{}{}",
            $color_code,
            $config.style.vertical,
            crate::RESET,
            pad,
            " ".repeat(available_content_width),
            pad,
            format!("{}{}{}", $color_code, $config.style.vertical, crate::RESET)
        )
    }};
}

/// PROTECTED: Render the tee divider immediately above the status bar.
#[macro_export]
macro_rules! status_divider_line {
    ($config:expr, $inner_width:expr, $color_code:expr) => {{
        format!(
            "{}{}{}{}{}",
            $color_code,
            $config.style.tee_left,
            $config.style.horizontal.repeat($inner_width),
            $config.style.tee_right,
            crate::RESET
        )
    }};
}

/// PROTECTED: Render the status content line with alignment and colors.
#[macro_export]
macro_rules! status_content_line {
    ($config:expr, $inner_width:expr, $color_code:expr, $text_color_code:expr, $status_color_code:expr, $alignment:expr, $status_text:expr) => {{
        let available_content_width = $inner_width.saturating_sub(2 * $config.width.h_padding);
        let mut status_display = $status_text.to_string();
        if $config.width.fixed_width.is_some()
            && crate::get_display_width(&status_display) > available_content_width
        {
            status_display = crate::truncate_with_ellipsis(&status_display, available_content_width);
        }

        let final_width = crate::get_display_width(&status_display);
        let debug_prefix = "";
        let (left_pad_inner, right_pad_inner) = match $alignment {
            "center" => {
                let space = available_content_width.saturating_sub(final_width + debug_prefix.len());
                let lp = space / 2;
                (lp, space.saturating_sub(lp))
            }
            "right" => {
                let space = available_content_width.saturating_sub(final_width + debug_prefix.len());
                (space, 0)
            }
            _ => (
                0,
                available_content_width.saturating_sub(final_width + debug_prefix.len()),
            ),
        };

        let status_line = format!(
            "{}{}{}{}",
            debug_prefix,
            " ".repeat(left_pad_inner),
            status_display,
            " ".repeat(right_pad_inner)
        );

        let status_code = if !$status_color_code.is_empty() {
            $status_color_code
        } else {
            $text_color_code
        };

        let colored_status = if status_code.is_empty() {
            status_line
        } else {
            format!("{}{}{}", status_code, status_line, crate::RESET)
        };

        let pad = " ".repeat($config.width.h_padding);
        format!(
            "{}{}{}{}{}{}{}",
            $color_code,
            $config.style.vertical,
            crate::RESET,
            pad,
            colored_status,
            pad,
            format!("{}{}{}", $color_code, $config.style.vertical, crate::RESET)
        )
    }};
}
