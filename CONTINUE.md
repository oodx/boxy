# Continue Log – perf/render-write

## Summary
- `RenderTarget` now streams into any `std::io::Write` sink; the CLI path locks `stdout` instead of allocating an intermediate `String`.
- Snapshot coverage for `render_to_string` lives in `tests/render_snapshots.rs` with fixtures under `tests/fixtures/` to catch accidental byte regressions.
- Roadmap synced (Milestone 1.7 marked complete; height milestone flagged as done) and README/docs updated to advertise streaming + height features.
- Bench snapshots refreshed earlier (`meta/snaps/`), full test suite continues to pass.

## Current Branch
- `perf/render-write`

## Latest Benchmarks
- Rounded: ~1.21 ms (down from ~1.39 ms).
- Normal-long: ~1.26 ms.
- Heavy-wide: ~1.18 ms.
- Baseline stored with `cargo bench --bench status_render -- --save-baseline buffer-stream`.

## What’s Next
- Refresh CLI `--help` output now that height/streaming features are standard (see new `[M2-010]`).
- Plan the temporary Vec adapter removal alongside the upcoming public API work (M2) and wire streaming helpers into BoxBuilder.
- Evaluate TOML vs JSON for theme configs and prepare the migration away from deprecated `serde_yaml` (new `[M3-010]`, `[M3-011]`).
- Add a `height` Cargo feature gate during the feature-flag milestone (task `[M5-002a]`).

## Handy Commands
```bash
# rebuild + tests
cargo fmt
cargo test

# run showcase
bin/test.sh run perfect

# benchmark and compare
cargo bench --bench status_render -- --baseline buffer-stream
```
