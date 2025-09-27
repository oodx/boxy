# CHINA-05A Phase 2 Implementation Plan
## API Surface Parity - Title/Icon/Divider Support

**Date:** 2025-09-27
**Phase:** CHINA-05A Phase 2 of 5
**Story Points:** 9 (LARGE)
**Priority:** HIGH (Pre-M3 Critical Path)

---

## Executive Summary

Extend the API builders (`api::layout`) to support title, icon, divider, and vertical padding features that currently exist in CLI but are missing from the programmatic API. This achieves feature parity between CLI (BoxyConfig) and API (BoxBuilder).

---

## Current State Analysis

### CLI Capabilities (BoxyConfig) Not Yet in API

1. **Title** - Renders inside body as first content line (src/visual/utils.rs:839-841, 869-871, 892-893)
2. **Icon** - Injects before first content line with intelligent emoji detection (src/visual/utils.rs:926-1011)
3. **Dividers** - Horizontal separators after title and before status (src/visual/utils.rs:915-924)
4. **Vertical Padding** - pad_before_title, pad_after_title, pad_before_status, pad_after_status

### BoxBuilder Structure (src/api/layout.rs:560-574)

**Currently Has:**
- header, footer, status, body components
- style, width/height constraints
- layout_mode, visibility

**Missing:**
- title field (distinct from header)
- icon field
- divider flags (4 total)
- vertical padding flags (4 total)

---

## Design Decisions

### 1. Title Support - Critical Design Choice

**CLI Behavior:** Title renders as **first line of body content**, NOT as separate header component.

**API Design:**
```rust
pub struct BodyBuilder {
    content: String,
    title: Option<String>,  // NEW - renders inside body
    icon: Option<String>,   // NEW - prepends to first line
    // ... existing fields (h_align, v_align, h_padding, v_padding, wrapping)
}

impl BodyBuilder {
    /// Set title that renders as first line of body content
    /// Matches CLI behavior: title appears INSIDE the box, not as header
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set icon that prepends to first content line
    /// Intelligent emoji detection prevents double-icons
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
}
```

**Rationale:**
- Title is body-level feature in CLI (Body::compose_content_lines)
- Keeps header/footer/status architecturally separate from title/icon
- Preserves progressive enhancement (titles optional)

### 2. Divider Support

**CLI Behavior:** Dividers render as horizontal lines with tee_left/horizontal/tee_right

**API Design:**
```rust
pub struct BoxBuilder {
    // ... existing fields
    divider_after_title: bool,       // NEW
    divider_before_status: bool,     // NEW
    pad_after_title_divider: bool,   // NEW
    pad_before_status_divider: bool, // NEW
}

impl BoxBuilder {
    /// Add horizontal divider after title line
    /// - padded: adds blank line after divider
    pub fn with_title_divider(mut self, padded: bool) -> Self {
        self.divider_after_title = true;
        self.pad_after_title_divider = padded;
        self
    }

    /// Add horizontal divider before status line
    /// - padded: adds blank line before divider
    pub fn with_status_divider(mut self, padded: bool) -> Self {
        self.divider_before_status = true;
        self.pad_before_status_divider = padded;
        self
    }
}
```

### 3. Vertical Padding Support

**CLI Behavior:** Adds blank lines before/after title and status sections

**API Design:**
```rust
pub struct BoxBuilder {
    // ... existing fields
    pad_before_title: bool,  // NEW
    pad_after_title: bool,   // NEW
    pad_before_status: bool, // NEW
    pad_after_status: bool,  // NEW
}

impl BoxBuilder {
    /// Add vertical padding around title
    /// - before: blank line before title
    /// - after: blank line after title
    pub fn with_title_padding(mut self, before: bool, after: bool) -> Self {
        self.pad_before_title = before;
        self.pad_after_title = after;
        self
    }

    /// Add vertical padding around status
    /// - before: blank line before status
    /// - after: blank line after status
    pub fn with_status_padding(mut self, before: bool, after: bool) -> Self {
        self.pad_before_status = before;
        self.pad_after_status = after;
        self
    }
}
```

---

## Progressive Enhancement Compliance

✅ **Layer 0 (Pure API)**: All new features optional via builder methods
✅ **Layer 1 (Config)**: Adapter will map BoxyConfig fields to new builders
✅ **Layer 2 (Theming)**: No changes needed - theming already supports title_color

**Example Usage:**
```rust
// Layer 0: Pure API with new features
use boxy::api::layout::{BoxBuilder, BodyBuilder};

let layout = BoxBuilder::new("Body content")
    .with_header(HeaderBuilder::new("Header"))
    .with_title("Title Line")  // NEW - renders inside body
    .with_icon("🔥")          // NEW - prepends to first line
    .with_title_divider(true) // NEW - adds divider after title
    .build();

// Layer 1: Config still works (adapter handles mapping)
let config = BoxyConfig {
    title: Some("Title".to_string()),
    icon: Some("🔥".to_string()),
    dividers: DividerConfig {
        divider_after_title: true,
        pad_after_title_divider: true,
        ..Default::default()
    },
    ..Default::default()
};
let layout = BoxLayout::from(&config);
```

---

## Implementation Tasks

### Task Breakdown (9 Story Points)

**[CHINA-05A-P2-01] Extend BodyBuilder (2 pts)**
- Add `title: Option<String>` field
- Add `icon: Option<String>` field
- Implement `with_title()` method
- Implement `with_icon()` method
- Update BodyBuilder::build() to handle title/icon

**[CHINA-05A-P2-02] Extend BoxBuilder (3 pts)**
- Add 4 divider boolean fields
- Add 4 vertical padding boolean fields
- Implement `with_title_divider(padded: bool)` method
- Implement `with_status_divider(padded: bool)` method
- Implement `with_title_padding(before, after)` method
- Implement `with_status_padding(before, after)` method
- Update BoxBuilder::build() to pass new fields to layout

**[CHINA-05A-P2-03] Update BoxLayout Rendering (2 pts)**
- Update BoxLayout::render() to handle dividers
- Update BoxLayout::render() to handle vertical padding
- Ensure title/icon render inside body (not as header)
- Preserve CLI behavior for icon emoji detection

**[CHINA-05A-P2-04] Update Config Adapter (1 pt)**
- Map config.icon → builder.with_icon()
- Map config.dividers.divider_after_title → builder.with_title_divider()
- Map config.dividers.divider_before_status → builder.with_status_divider()
- Map config.padding fields → builder padding methods

**[CHINA-05A-P2-05] Add Tests (1 pt)**
- Test title rendering inside body
- Test icon prepending with emoji detection
- Test dividers after title and before status
- Test vertical padding placement
- Test config adapter mapping

**[CHINA-05A-P2-06] Update Documentation (0.5 pts)**
- Add "Title and Icon Support" section to API_README.md
- Add "Dividers" section with examples
- Add "Vertical Padding" section with examples
- Update config adapter documentation

---

## Files to Modify

### Primary Changes

**src/api/layout.rs** (Major)
- Extend BodyBuilder struct (lines ~480-520)
- Add with_title(), with_icon() methods
- Extend BoxBuilder struct (lines ~560-574)
- Add 8 new builder methods for dividers/padding
- Update BoxLayout::build() logic (~lines 700-850)

**src/api/config.rs** (Moderate)
- Update From<&BoxyConfig> for BoxLayout (lines 77-150)
- Map new fields from BoxyConfig to builder methods

**API_README.md** (Documentation)
- Add 3 new sections with examples

### Testing

**src/api/layout.rs** (tests module)
- Add 5-7 new tests for feature validation

---

## Validation Criteria

### Success Criteria
✅ All 124+ existing tests pass
✅ Title renders inside body (first line), not as header
✅ Icon prepends to first content line with emoji detection
✅ Dividers render as horizontal tee lines
✅ Vertical padding adds blank lines correctly
✅ Config adapter maps all new fields correctly
✅ API_README.md documents new features with examples

### Regression Prevention
- Title must NOT render outside body (BUG-01 regression check)
- Icon detection must match CLI logic (avoid double emojis)
- Width calculations must remain Unicode-aware
- All color/theming remains opt-in (progressive enhancement)

---

## Risk Assessment

**Low Risk Areas:**
- Divider rendering (well-understood pattern)
- Vertical padding (simple blank line insertion)
- Config adapter updates (straightforward mapping)

**Medium Risk Areas:**
- Title placement (must match CLI behavior exactly)
- Icon rendering (emoji detection logic must be preserved)

**Mitigation:**
- Reference CLI implementation (src/visual/utils.rs:926-1011)
- Add comprehensive tests for title/icon edge cases
- Validate against existing CLI outputs

---

## Dependencies

**Blocked By:** None (CHINA-05A Phase 1 complete)
**Blocks:** CHINA-05A Phase 3 (CLI switchover requires complete API parity)
**Related:** QOL-02 (per-component theming already supports title_color)

---

## Timeline

**Estimated Duration:** 2-3 hours
**Parallelization:** China can update TASKS.txt/CONTINUE.md while implementation proceeds

---

## Next Steps

1. Update TASKS.txt with 6 sub-tickets
2. Implement in priority order: BodyBuilder → BoxBuilder → Rendering → Adapter → Tests → Docs
3. Run full test suite after each major change
4. Have China update status docs when complete
5. Commit with descriptive message documenting Phase 2 completion