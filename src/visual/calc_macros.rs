//! Status calculation macros - centralized to prevent accidental regressions.
//!
//! These macros encapsulate the exact padding/divider/rendering math that keeps
//! the status bar aligned with the rest of the box. They intentionally mirror
//! the proven logic from the last-working build so any future edits happen in a
//! single protected location.

/// PROTECTED: Render a blank padding line around the status bar.
#[macro_export]
macro_rules! status_padding_line {
    ($config:expr, $color_code:expr, $pad:expr, $blank:expr) => {{
        let mut line = String::with_capacity(
            $color_code.len() * 2
                + $crate::RESET.len() * 2
                + $config.style.vertical.len() * 2
                + $pad.len() * 2
                + $blank.len(),
        );
        line.push_str($color_code);
        line.push_str($config.style.vertical);
        line.push_str($crate::RESET);
        line.push_str($pad);
        line.push_str($blank);
        line.push_str($pad);
        line.push_str($color_code);
        line.push_str($config.style.vertical);
        line.push_str($crate::RESET);
        line
    }};
    ($config:expr, $inner_width:expr, $color_code:expr) => {{
        let pad = " ".repeat($config.width.h_padding);
        let blank = " ".repeat($inner_width.saturating_sub(2 * $config.width.h_padding));
        $crate::status_padding_line!($config, $color_code, pad.as_str(), blank.as_str())
    }};
}

/// PROTECTED: Render the tee divider immediately above the status bar.
#[macro_export]
macro_rules! status_divider_line {
    ($config:expr, $color_code:expr, $horizontal:expr) => {{
        let mut line = String::with_capacity(
            $color_code.len() * 2
                + $crate::RESET.len()
                + $config.style.tee_left.len()
                + $config.style.tee_right.len()
                + $horizontal.len(),
        );
        line.push_str($color_code);
        line.push_str($config.style.tee_left);
        line.push_str($horizontal);
        line.push_str($config.style.tee_right);
        line.push_str($crate::RESET);
        line
    }};
    ($config:expr, $inner_width:expr, $color_code:expr) => {{
        let horizontal = $config.style.horizontal.repeat($inner_width);
        $crate::status_divider_line!($config, $color_code, horizontal.as_str())
    }};
}

/// PROTECTED: Render the status content line with alignment and colors.
#[macro_export]
macro_rules! status_content_line {
    ($config:expr, $color_code:expr, $text_color_code:expr, $status_color_code:expr, $alignment:expr, $status_text:expr, $pad:expr, $available_width:expr) => {{
        let mut status_display = $status_text.to_string();
        if $config.width.fixed_width.is_some()
            && $crate::get_display_width(&status_display) > $available_width
        {
            status_display = $crate::truncate_with_ellipsis(&status_display, $available_width);
        }

        let final_width = $crate::get_display_width(&status_display);
        let (left_pad_inner, right_pad_inner) = match $alignment {
            "center" => {
                let space = $available_width.saturating_sub(final_width);
                let lp = space / 2;
                (lp, space.saturating_sub(lp))
            }
            "right" => {
                let space = $available_width.saturating_sub(final_width);
                (space, 0)
            }
            _ => (0, $available_width.saturating_sub(final_width)),
        };

        let mut status_line =
            String::with_capacity(left_pad_inner + status_display.len() + right_pad_inner);
        status_line.extend(std::iter::repeat(' ').take(left_pad_inner));
        status_line.push_str(&status_display);
        status_line.extend(std::iter::repeat(' ').take(right_pad_inner));

        let status_code = if !$status_color_code.is_empty() {
            $status_color_code
        } else {
            $text_color_code
        };

        let colored_status = if status_code.is_empty() {
            status_line
        } else {
            let mut colored =
                String::with_capacity(status_code.len() + status_line.len() + $crate::RESET.len());
            colored.push_str(status_code);
            colored.push_str(&status_line);
            colored.push_str($crate::RESET);
            colored
        };

        let mut line = String::with_capacity(
            $color_code.len() * 2
                + $crate::RESET.len()
                + $config.style.vertical.len() * 2
                + $pad.len() * 2
                + colored_status.len(),
        );
        line.push_str($color_code);
        line.push_str($config.style.vertical);
        line.push_str($crate::RESET);
        line.push_str($pad);
        line.push_str(&colored_status);
        line.push_str($pad);
        line.push_str($color_code);
        line.push_str($config.style.vertical);
        line.push_str($crate::RESET);
        line
    }};
    ($config:expr, $inner_width:expr, $color_code:expr, $text_color_code:expr, $status_color_code:expr, $alignment:expr, $status_text:expr) => {{
        let pad = " ".repeat($config.width.h_padding);
        let available_width = $inner_width.saturating_sub(2 * $config.width.h_padding);
        $crate::status_content_line!(
            $config,
            $color_code,
            $text_color_code,
            $status_color_code,
            $alignment,
            $status_text,
            pad.as_str(),
            available_width
        )
    }};
}
