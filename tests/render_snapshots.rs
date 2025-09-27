use boxy::{BoxColors, BoxStyle, BoxyConfig, WidthConfig};
use boxy::api::layout::BoxLayout;

fn snapshot_config_basic() -> BoxyConfig {
    let mut config = BoxyConfig::default();
    config.text = "Hello Boxy!\nLine 2".to_string();
    config.title = Some("Snapshot Test".to_string());
    config.status_bar = Some("sr:Status: OK".to_string());
    config.width = WidthConfig {
        fixed_width: Some(28),
        ..WidthConfig::default()
    };
    config.colors = BoxColors {
        box_color: "none".to_string(),
        text_color: "none".to_string(),
        title_color: None,
        status_color: None,
        ..BoxColors::default()
    };
    config.style = BoxStyle::default();
    config
}

#[test]
fn snapshot_basic_box() {
    let config = snapshot_config_basic();
    let actual = BoxLayout::from(&config).render();
    let expected = include_str!("fixtures/render_basic.txt");
    assert_eq!(actual, expected);
}
