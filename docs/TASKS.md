# Boxy Implementation Tasks

## Story Point Scale
- **1 point**: Trivial change (<30 min)
- **2 points**: Simple feature (30-60 min)
- **3 points**: Moderate complexity (1-2 hours)
- **5 points**: Complex feature (2-4 hours)
- **8 points**: Major feature (4-8 hours)

---

## ✅ COMPLETED SPRINTS

### ✅ Sprint 1: Core Visual Features (16/16 points) - COMPLETED
- ✅ `--title` flag with truncation and variable expansion
- ✅ `--footer` flag with same capabilities as title  
- ✅ `--icon` flag with emoji and color support
- ✅ `--icon-pos` option for left/right positioning
- ✅ Modular draw_box refactoring

### ✅ Sprint 2: Pipeline Features (13/13 points) - COMPLETED
- ✅ `--no-boxy` default mode for content extraction
- ✅ `--no-boxy=strict` mode for pure ASCII output
- ✅ Input detection and error handling

### ✅ Sprint 3: Width Control (8/8 points) - COMPLETED
- ✅ `--width`/`-w` flag implementation
- ✅ Content truncation with Unicode ellipsis (`…`)
- ✅ Title/footer ellipsis upgrade from `...` to `…`

### ✅ Sprint 4: Theme System & Text Colors (12/12 points) - COMPLETED  
- ✅ `--theme` flag with predefined visual styles
- ✅ Theme system with error, success, warn, info, debug themes
- ✅ `--text` flag for independent text color control
- ✅ Auto-mode (`--text auto`) for box color matching
- ✅ Unified icon handling approach for consistent spacing
- ✅ Integration of themes with manual color/icon overrides

---

## Sprint 5: Advanced Width Features (Total: 12 points)

### Task 5.1: Smart word wrapping within fixed width [5 points]
- [ ] Implement word-boundary wrapping logic
- [ ] Handle long words that exceed line width
- [ ] Preserve ANSI color codes across line breaks
- [ ] Test with various content types (code, prose, mixed)
- [ ] Maintain consistent padding and alignment

### Task 5.2: Content overflow enhancements [3 points]
- [ ] Add overflow indicators beyond ellipsis
- [ ] Implement multi-line balancing algorithms
- [ ] Handle edge cases (empty lines, whitespace-only lines)

### Task 5.3: Responsive width features [4 points]
- [ ] Terminal width detection (`tput cols`)
- [ ] Percentage-based width calculations (`--width=50%`)
- [ ] Auto-fit content with max width limits
- [ ] Fallback behavior for non-interactive terminals

---

## Sprint 6: Content Enhancement (Total: 10 points)

### Task 6.1: Advanced content handling [3 points]
- [ ] Better Unicode character width calculations
- [ ] Support for zero-width characters
- [ ] Emoji variant handling (text vs graphical presentation)

### Task 6.2: Layout improvements [3 points]
- [ ] Vertical alignment options (top, center, bottom)
- [ ] Horizontal content justification (left, center, right, justify)
- [ ] Configurable padding per side

### Task 6.3: Content filtering and processing [4 points]
- [ ] Tab expansion with configurable width
- [ ] Whitespace normalization options
- [ ] Content preprocessing hooks

---

## Sprint 7: Polish & Optimization (Total: 8 points)

### Task 7.1: Performance optimization [3 points]
- [ ] Cache width calculations where possible
- [ ] Optimize for common ASCII case
- [ ] Profile and reduce memory allocations
- [ ] Benchmark against large inputs

### Task 7.2: Enhanced color palette [3 points]
- [ ] Add 256-color palette support
- [ ] Support hex color codes (`#FF5733`)
- [ ] Add semantic color aliases (`error`, `success`, `warning`)
- [ ] Expand theme preset system

### Task 7.3: Error handling & validation [2 points]
- [ ] Better error messages for invalid inputs
- [ ] Graceful degradation for terminal limitations
- [ ] Input validation with helpful suggestions

---

## Backlog (Future Sprints)

### Multi-column support [8 points]
### Configuration file support [5 points]
### Shell completions [3 points each]
### Interactive resize mode [8 points]
### Template system [5 points]

---

## Quick Start Order

For immediate implementation, prioritize:
1. Task 1.5 (refactor) - Sets foundation
2. Task 1.1 (title) - Most requested feature
3. Task 2.1 (no-boxy) - Enables pipelines
4. Task 1.2 (footer) - Completes visual symmetry
5. Task 2.2 (strict) - Completes pipeline story

**Estimated Phase 1 completion**: 6-8 hours of focused development