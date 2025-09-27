# Continue Log – main + Library API Development

## Summary
- Created new `src/api/` module for Room Runtime integration with pure geometry/layout capabilities
- CODEX review identified 3 CRITICAL regressions that must be fixed before Room Runtime use
- Sprint updated to include API fixes as highest priority (9 story points)
- Tasks added to SPRINT.txt and TASKS.txt tracking boards

## Current Branch
- `main`

## Latest Work
- Library API module created (`src/api/`) with geometry, layout, and theming submodules
- Background color feature added (ANSI, RGB, Named, Hex support)
- Per-component theming hooks implemented (`apply_component_colors()`)
- Title handling regression fixed in CLI→API adapter
- China review: 4.5/5 stars on architecture
- CODEX review: Found 3 critical regressions that need immediate fixes (2 resolved)

## Latest Benchmarks
- Rounded: ~1.21 ms (down from ~1.39 ms).
- Normal-long: ~1.26 ms.
- Heavy-wide: ~1.18 ms.
- Baseline stored with `cargo bench --bench status_render -- --save-baseline buffer-stream`.

## Technical Debt Analysis (NEW)
- **China's Golden Egg**: Project architecture wisdom consolidated into comprehensive FAQ
- **Tina's Red Egg**: RSB compliance (35% → 90% needed) + test coverage analysis via test.sh
- **Task Tickets Created**:
  - CHINA-01 through CHINA-12 (architecture, performance, UX debt)
  - TEST-01 through TEST-18 (RSB compliance, testing infrastructure, automation)
- **YAML→TOML Migration Plan**: Complete strategy documented in `docs/YAML_TOML_MIGRATION.txt`

## Pre-M3 Critical Path
High priority technical debt that should be addressed before M3:
- **CHINA-01**: Theme System Architecture Cleanup (LARGE effort)
- **CHINA-02**: Theme Inheritance Engine Critical Bug Fix (MEDIUM effort)
- **CHINA-05**: Library API Ergonomics Enhancement (LARGE effort)
- **TEST-01, TEST-02, TEST-03**: Benchmark integration & performance regression detection
- **TEST-05 through TEST-08**: RSB architecture compliance (65% gap to close)

## What's Next (Priority Order)
- **[API-FIX-01]**: Fix missing closing borders in Body/Status (CRITICAL)
- **[API-FIX-02]**: Fix BoxBuilder default open box issue (CRITICAL)
- **[API-FIX-03]**: Replace custom truncation with Unicode-safe utility (HIGH)
- **[API-TEST-01]**: Add comprehensive geometry assertions (HIGH)
- **[CHINA-02]**: Fix theme inheritance engine critical bug (after API fixes)
- **[TEST-01]**: Integrate benchmarks into test.sh workflow

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
