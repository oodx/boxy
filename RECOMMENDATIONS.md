# Boxy v0.6.x – Recommendations (Cheap SP Wins)

## Immediate (Low Effort)

- Icon mode flag (1 SP)
  - `--icon=auto|none` to explicitly suppress icon injection when `none`.

- Header/Footer colors (2 SP)
  - Flags: `--header-color <COLOR>`, `--footer-color <COLOR>`
  - Params: `hc='color'`, `fc='color'`
  - Ignore on invalid color; fall back gracefully.

- Title/Status text styles (2 SP)
  - Flags: `--title-style <STYLE>`, `--status-style <STYLE>` (bold|italic|underline|dim)
  - Params: `ts='style'`, `ss='style'`
  - ANSI-only, no layout implications.

- Body wrap mode (2 SP)
  - `--wrap` to wrap lines within inner width (instead of truncation).
  - Coexists with alignment and emoji pad.

- Box margin (1 SP)
  - `--margin a|b` (above|below) to add a blank line before/after the entire box.
  - Extensible later to numeric counts (e.g., `a=2,b=1`).

- Default layout via env (1 SP)
  - `BOXY_LAYOUT` applied when `--layout`/params `ly` absent.

- Default jynx toggle via env (1 SP)
  - `BOXY_JYNX=0` to disable jynx enhancement without `--no-color`.

- Colors view filters (1 SP)
  - `--colors --filter "<substring>"` to show matching color names in columns.

- Title/Status max width trims (1 SP)
  - `--title-max N`, `--status-max N` to constrain those lines independently.

- UAT polish – timing (1 SP)
  - Capture per-test duration + exit code; show in mega box footer.

## Short Backlog

- Params ha/fa (1 SP)
  - `ha=`/`fa=` for header/footer alignment; maps to existing layout.

- Numeric color aliases + names-only view (3 SP)
  - Short numeric aliases (e.g., `00=black`); `--colors --names` for names-only columns.

- Table/multi-row “multi-boxy” (3–4 SP)
  - Row rendering using cross tees; foundation for tabular views.

## Notes

- Current features include: params `ly`, title/status color (`tc`/`sc`), body align (`bl|bc|br`), emoji pad (`bp`), general padding (`--pad a|b`), width keywords (`max|auto`), and `BOXY_THEME` default.
- All proposed items reuse existing parsing/render hooks and follow the same graceful-degradation principles.

