use boxy::{
    AlignmentConfig, Body, BoxColors, BoxStyle, BoxyConfig, DividerConfig, Footer, HEAVY, Header,
    NORMAL, PaddingConfig, ROUNDED, Status, WidthConfig, calculate_box_width, get_color_code,
};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn build_config(template: &str, style: BoxStyle) -> BoxyConfig {
    let mut config = BoxyConfig::default();

    config.text = template.to_string();
    config.title = Some("Benchmark Title".to_string());
    config.footer = Some("Benchmark Footer".to_string());
    config.header = Some("Benchmark Header".to_string());
    config.status_bar = Some("sr:⚡ Benchmark status ready".to_string());
    config.icon = Some("🚀".to_string());

    config.width = WidthConfig {
        fixed_width: Some(60),
        h_padding: 2,
        v_padding: 1,
        enable_wrapping: false,
    };

    config.fixed_height = Some(20);

    config.padding = PaddingConfig {
        pad_before_title: true,
        pad_after_title: true,
        pad_before_status: true,
        pad_after_status: true,
        pad_body_above: false,
        pad_body_below: false,
    };

    config.dividers = DividerConfig {
        divider_after_title: true,
        divider_before_status: true,
        pad_after_title_divider: true,
        pad_before_status_divider: true,
    };

    config.alignment = AlignmentConfig {
        header_align: "center".to_string(),
        footer_align: "center".to_string(),
        status_align_override: None,
    };

    config.colors = BoxColors {
        box_color: "blue".to_string(),
        text_color: "white".to_string(),
        title_color: Some("azure".to_string()),
        status_color: Some("lime".to_string()),
        header_color: None,
        footer_color: None,
    };

    config.style = style;
    config
}

fn text_color_code(config: &BoxyConfig, color_code: &'static str) -> &'static str {
    match config.colors.text_color.as_str() {
        "auto" => color_code,
        "none" => "",
        other => get_color_code(other),
    }
}

fn render_full(config: &BoxyConfig) -> Vec<String> {
    let mut lines = Vec::new();

    let mut all_content = config.text.clone();
    if let Some(title) = &config.title {
        all_content.push('\n');
        all_content.push_str(title);
    }

    let final_width = calculate_box_width(
        &all_content,
        config.width.h_padding,
        config.width.fixed_width,
        config.width.enable_wrapping,
    );
    let inner_width = final_width.saturating_sub(2);
    let color_code = get_color_code(&config.colors.box_color);
    let text_color_code = text_color_code(config, color_code);
    let title_color_code = config
        .colors
        .title_color
        .as_ref()
        .map(|name| get_color_code(name))
        .unwrap_or("");
    let status_color_code = config
        .colors
        .status_color
        .as_ref()
        .map(|name| get_color_code(name))
        .unwrap_or("");

    let header = Header::new(config);
    lines.push(header.render(inner_width, color_code));

    let body = Body::new(config);
    let body_lines = body.render(inner_width, color_code, text_color_code, title_color_code);
    let body_len = body_lines.len();
    lines.extend(body_lines);

    let status = Status::new(config);
    let mut status_lines = Vec::new();
    if status.should_render() {
        status_lines = status.render(inner_width, color_code, text_color_code, status_color_code);
        lines.extend(status_lines.iter().cloned());
    }

    if let Some(target_height) = config.fixed_height {
        let current_total = 1 + body_len + status_lines.len() + 1;
        if target_height > current_total {
            let filler_needed = target_height - current_total;
            let pad = " ".repeat(config.width.h_padding);
            let available_content_width = inner_width.saturating_sub(2 * config.width.h_padding);
            let blank = " ".repeat(available_content_width);
            for _ in 0..filler_needed {
                lines.push(boxy::status_padding_line!(
                    config,
                    color_code,
                    pad.as_str(),
                    blank.as_str()
                ));
            }
        }
    }

    let footer = Footer::new(config);
    lines.push(footer.render(inner_width, color_code));

    lines
}

fn bench_group(c: &mut Criterion) {
    let cases = [
        (
            "rounded",
            ROUNDED,
            "This is a short status render benchmark.",
        ),
        (
            "normal-long",
            NORMAL,
            "Line one for benchmark\nLine two with emoji 🚀\nLine three for padding",
        ),
        (
            "heavy-wide",
            HEAVY,
            "Benchmark with heavy borders and extended content for stress testing.",
        ),
    ];

    let mut group = c.benchmark_group("render_full");

    for (label, style, template) in cases {
        let config = build_config(template, style);
        group.bench_function(BenchmarkId::new("render", label), |b| {
            b.iter(|| {
                let lines = render_full(&config);
                black_box(lines);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_group);
criterion_main!(benches);
