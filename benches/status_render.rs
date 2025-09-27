use boxy::{
    AlignmentConfig, BoxColors, BoxStyle, BoxyConfig, DividerConfig, HEAVY, NORMAL, PaddingConfig,
    ROUNDED, WidthConfig,
};
use boxy::api::layout::BoxLayout;
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
                let rendered = BoxLayout::from(&config).render();
                black_box(rendered);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_group);
criterion_main!(benches);
