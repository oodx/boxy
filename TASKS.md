# Boxy Implementation Tasks

## Story Point Scale
- **1 point**: Trivial change (<30 min)
- **2 points**: Simple feature (30-60 min)
- **3 points**: Moderate complexity (1-2 hours)
- **5 points**: Complex feature (2-4 hours)
- **8 points**: Major feature (4-8 hours)

---

## Sprint 1: Core Visual Features (Total: 16 points)

### Task 1.1: Implement `--title` flag [5 points]
- [ ] Add title argument parsing
- [ ] Calculate title placement in top border
- [ ] Implement truncation with `...`
- [ ] Support color codes in title text
- [ ] Handle environment variable expansion
- [ ] Test with various box styles and widths

### Task 1.2: Implement `--footer` flag [3 points]
- [ ] Reuse title rendering logic for bottom border
- [ ] Add footer argument parsing
- [ ] Test truncation and positioning
- [ ] Ensure compatibility with all box styles

### Task 1.3: Implement `--icon` flag [3 points]
- [ ] Add icon argument parsing
- [ ] Calculate icon placement (default: top-left)
- [ ] Support color wrapping for characters
- [ ] Handle emoji width calculations

### Task 1.4: Add `--icon-pos` option [2 points]
- [ ] Parse position argument (left|right)
- [ ] Adjust icon rendering based on position
- [ ] Update help documentation

### Task 1.5: Refactor draw_box for modularity [3 points]
- [ ] Extract border rendering functions
- [ ] Create shared title/footer rendering logic
- [ ] Improve code reusability

---

## Sprint 2: Pipeline Features (Total: 13 points)

### Task 2.1: Implement `--no-boxy` default mode [5 points]
- [ ] Parse box-stripping flag
- [ ] Extract content from boxed input
- [ ] Remove box characters and padding
- [ ] Preserve internal formatting and colors
- [ ] Handle multi-line boxed content

### Task 2.2: Implement `--no-boxy=strict` mode [5 points]
- [ ] Add strict mode parsing
- [ ] Strip all ANSI escape codes
- [ ] Remove emoji and Unicode decorations
- [ ] Trim whitespace aggressively
- [ ] Output pure ASCII text

### Task 2.3: Add input detection logic [3 points]
- [ ] Detect if input is already boxed
- [ ] Handle edge cases (partial boxes, nested boxes)
- [ ] Provide appropriate error messages

---

## Sprint 3: Testing & Documentation (Total: 8 points)

### Task 3.1: Create comprehensive test suite [5 points]
- [ ] Unit tests for each new flag
- [ ] Integration tests for flag combinations
- [ ] Pipeline tests (box -> no-boxy -> box)
- [ ] Edge case testing (long titles, special chars)
- [ ] Performance benchmarks

### Task 3.2: Update documentation [2 points]
- [ ] Update --help text with new options
- [ ] Add examples for each feature
- [ ] Create man page if needed

### Task 3.3: Add CI/CD validation [1 point]
- [ ] Ensure tests pass in GitHub Actions
- [ ] Add new feature examples to test.sh

---

## Sprint 4: Polish & Optimization (Total: 7 points)

### Task 4.1: Optimize Unicode width calculations [3 points]
- [ ] Cache width calculations where possible
- [ ] Optimize for common ASCII case
- [ ] Profile and reduce allocations

### Task 4.2: Add color palette expansion [2 points]
- [ ] Add more color options
- [ ] Support hex color codes (stretch goal)
- [ ] Add color aliases (e.g., "error" -> "red")

### Task 4.3: Improve error handling [2 points]
- [ ] Better error messages for invalid inputs
- [ ] Graceful degradation for terminal limitations
- [ ] Add --validate flag for testing

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