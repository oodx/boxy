# Boxy Library QoL Improvements

These ideas focus on polishing the public API so downstream consumers (Room Runtime and others) can build richer experiences with less boilerplate.

## Quick Wins

- **Clarify style impact in geometry API**
  `geometry::calculate_box_dimensions` accepts a `BoxStyle` whose ANSI attributes can alter width calculations even when borders are single-width (`src/api/geometry.rs:79-178`). Document this behavior and add regression tests covering styles with ANSI decoration so callers understand why the argument matters.

- **Offer a convenience renderer**
  Most clients repeat `BoxBuilder::new(content).with_header(...).build().render()`. A helper like `layout::render_box(content, BoxOptions)` (defaults + optional title/status) would cover the 80% use case and keep the builder available for advanced scenarios.

- **Expose line-by-line rendering**
  `BoxLayout::render()` always joins to a single `String`. Add a `render_lines()` (or `into_lines()`) that returns `Vec<String>` so layout engines don’t need to split strings back into lines when positioning components.

## Medium Effort

- **Per-component theming hooks**
  `theming::apply_colors` wraps the full rendered string, which makes per-section styling cumbersome. Consider a `Renderer` trait or callbacks that accept each `ComponentLayout`, allowing consumers to color headers/body/status independently without re-parsing the Unicode art.

- **Room Runtime adapter struct**
  Provide a helper that emits component offsets (row ranges, heights, widths) alongside the string content. That avoids duplicated geometry math across host applications.

- **High-level presets**
  Introduce a `BoxTemplate`/`PresetTheme` registry with ergonomic constructors like `layout::BoxBuilder::preset("status")`. This would encapsulate common header/footer/status combinations while leaving the underlying builders intact for custom layouts.

## Longer-Term Enhancements

- **Typed error surface**
  When Milestone 4 lands, expose the new error enums through the API so library consumers get `Result<T, BoxyError>` instead of panics (especially for future theme/IO functions).

- **Background/foreground composition helpers**
  Background colors exist, but mixing them with foreground themes requires manual string surgery. Provide helper methods on `ComponentLayout` (e.g., `with_background(bg)`) or richer renderer options to apply backgrounds selectively.

- **One-shot geometry + layout wrappers**
  Add ergonomic wrappers such as `PlainBox::new(content).width(40).render()` that internally call geometry + layout. These could live alongside the builder API for clients who just need “box around text” without touching low-level pieces.
