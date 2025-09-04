# Boxy Theme Upgrade Proposal (v0.6.x → v0.7)

## Goals

- Make YAML themes first‑class and dynamic, aligned with jynx.
- Keep a compiled default/base theme so Boxy runs with zero setup.
- Allow a shared theme pool for jynx + boxy without code changes.
- Preserve a legacy path (opt‑in) for baked themes to ease migration.
- Move emoji/icon and visual tokens out of Rust and into YAML.

## Requirements (from product)

- Compiled theme(s) sourced from `./themes` at build time (no code mucking to switch).
- Load from the XDG+ themes folder at runtime in addition to compiled defaults.
- Inheritance/prioritization similar to jynx (start from base, override or extend).
- Don’t change every time the user tweaks a file — compiled default is the global fallback.

## Resolution Order & Precedence

1) User themes: `~/.local/etc/rsb/boxy/themes/*.yml`
2) Shared (for jynx + boxy), e.g.:
   - `~/.local/etc/rsb/shared-themes/*.yml` (preferred)
   - Or `~/.local/etc/rsb/jynx/themes/*.yml`
3) System themes: `/etc/xdg/rsb/boxy/themes/*.yml`
4) Compiled themes: embedded at build time from `./themes/*.yml`
5) Legacy baked themes: only if `--legacy` is set (safety net)

First match wins; when a theme is found, apply inheritance within the resolved file(s).

## Inheritance & Priority Model

- Reuse Boxy’s current YAML `inherits` support (child overrides parent).
- Encourage a “base” theme (e.g., `base.yml`) extended by specific themes (error/success/etc.).
- Support multiple theme files; `inherits` may reference a theme from any loaded file.

## Build‑time Embedding (no runtime code changes needed to choose files)

- Add `build.rs` to scan `./themes/*.yml` at build time.
- Generate an internal module (e.g., `embedded_themes.rs`) using `include_str!()` for each file.
- Compile a static registry: `EMBEDDED_THEMES: &[(name: &'static str, yaml: &'static str)]`.
- Default/base theme selection by naming convention (no Rust edits):
  - Use `./themes/default.yml` if present; else `./themes/theme-boxy.yml`.
  - Optionally accept `DEFAULT_THEME=<name>` env at build time to override (optional, not required).

## Runtime Behavior

- ThemeEngine initializes with embedded themes available as the final fallback.
- On first run: if no user themes exist, write the selected embedded default to the user XDG dir with a friendly message (optional, but nice UX).
- `BOXY_THEME` env respected as the default if `--theme` is not provided.
- Add `--legacy` to opt into old baked themes (deprecated path; warn).

## CLI Enhancements (to complete the loop)

- `boxy theme show <name>`: pretty prints the resolved YAML (or nearest exact theme) with source hints.
- `boxy theme import <path>`: imports YAML to `~/.local/etc/rsb/boxy/themes/` (already present, polish messages).
- `boxy theme export <name>`: prints YAML (already present).
- `boxy theme validate <path>`: validate YAML file without importing (cheap).

## Emoji/Icon Source of Truth

- Move icon/emoji and title iconization into YAML (per theme), not Rust.
- Keep a tiny alias map if desired (e.g., `icon: check` → `✅`) for readability, but YAML strings will do.

## Compatibility

- Legacy baked themes disabled by default; gate under `--legacy` (and warn).
- Keep the current theme names available as compiled YAML (from `./themes`), so users aren’t broken.

## Implementation Plan & SP

1) Theme show + import/export polish (2 SP)
   - Implement `theme show` with source hints; tidy import/export output.

2) Build‑time embedded defaults (3 SP)
   - Add `build.rs` to embed `./themes/*.yml`.
   - Select default from `default.yml` or `theme-boxy.yml` (by file name).

3) Legacy opt‑in (2 SP)
   - Add `--legacy` and gate old baked themes behind it with warnings.

4) Shared theme resolution (2 SP)
   - Add shared XDG dir for jynx + boxy; insert it between user and system.

5) YAML icon/emoji canonicalization (2 SP)
   - Move defaults to YAML; optional alias mapping.

Total: ~11 SP (without live reload/watch).

## Future (Optional)

- Live reload of YAML themes (watch files) – 2–3 SP
- Central theme registry sync – 2 SP
- Numeric color aliases & names‑only colors view – 3 SP

## Notes

- This approach allows you to “configure the compiled default” purely by placing a YAML file in `./themes` before build — no Rust code editing.
- It aligns with jynx’s dynamic ethos while giving Boxy a sensible no‑setup path.
- We already have the theme engine and XDG directories; this proposal is mostly wiring and smoothing the UX.
