# CHINA-05A Phase 3: CLI Switchover Implementation Plan

## Overview
Migrate CLI from legacy `draw_box()` function to new BoxBuilder API + config adapter pattern while maintaining 100% backward compatibility and preserving protected icon logic.

## Current Architecture Analysis

### Legacy System (main.rs → visual/utils.rs)
1. **main.rs**: Parses CLI args → builds `BoxyConfig` → calls `draw_box(config)`
2. **visual/utils.rs**: `draw_box()` → `render_box_with_width()` → legacy rendering
3. **Protected Icon Logic**: Lines 717-736 in main.rs use `apply_icon_to_text!` macro

### New API System (api/config.rs → api/layout.rs)
1. **api/config.rs**: `From<&BoxyConfig> for BoxLayout` adapter
2. **api/layout.rs**: BoxBuilder pattern with `.render()` method
3. **Icon Support**: Built into BodyBuilder with smart emoji detection

## Phase 3 Implementation Plan

### Sub-Task Breakdown (7 tickets, ~10 story points)

#### **[P3-01] Create CLI Migration Adapter (2 pts) - Critical**
- **Goal**: Bridge CLI argument parsing to new API system
- **Location**: `src/cli_adapter.rs` (new file)
- **Tasks**:
  - Create `CLIToAPI` struct that holds parsed CLI arguments
  - Implement `impl From<CLIToAPI> for BoxLayout`
  - Handle theme integration through adapter
  - Preserve existing CLI argument semantics exactly
- **Critical**: Must handle protected icon logic transition

#### **[P3-02] Preserve Protected Icon Logic (2 pts) - Critical**
- **Goal**: Ensure icon positioning works identically in new system
- **Location**: CLI adapter + API integration
- **Tasks**:
  - Map `apply_icon_to_text!` macro behavior to API BodyBuilder
  - Test icon spacing in Windows Terminal (critical requirement)
  - Validate theme icon integration continues working
  - Add regression tests for icon positioning edge cases
- **Risk**: Icon spacing was "NIGHTMARE to get right" - must be perfect

#### **[P3-03] Update Main.rs CLI Flow (2 pts) - High**
- **Goal**: Switch main.rs from `draw_box()` to new API pipeline
- **Location**: `src/main.rs` lines 742-777
- **Tasks**:
  - Replace `draw_box(config)` call with new adapter flow
  - Keep all argument parsing identical (no behavior changes)
  - Handle theme/color integration through new system
  - Preserve error handling and messaging
- **Dependency**: Requires P3-01 and P3-02 completion

#### **[P3-04] Theme System Integration (1 pt) - Medium**
- **Goal**: Wire theme/color handling through new component metadata
- **Location**: Theme engine → API integration
- **Tasks**:
  - Map theme application to BoxBuilder API
  - Ensure theme colors work with new rendering system
  - Validate theme icon logic preservation
  - Test theme inheritance behavior
- **Dependency**: Requires P3-01 completion

#### **[P3-05] Add Comprehensive Regression Tests (2 pts) - High**
- **Goal**: Ensure CLI flags behave identically in new system
- **Location**: `tests/cli_switchover_regression.rs` (new file)
- **Tasks**:
  - Test all CLI flag combinations produce identical output
  - Test protected icon logic edge cases
  - Test theme integration behavior
  - Test error cases and messaging
  - Create golden output comparison tests
- **Coverage**: Must test every CLI flag combination

#### **[P3-06] Performance Validation (0.5 pts) - Medium**
- **Goal**: Ensure new system doesn't degrade performance
- **Location**: Benchmark comparison
- **Tasks**:
  - Benchmark old vs new rendering performance
  - Validate no regression in emoji width calculations
  - Test memory usage patterns
  - Run existing performance test scripts
- **Acceptance**: No >5% performance degradation

#### **[P3-07] Documentation Update (0.5 pts) - Low**
- **Goal**: Update internal documentation for new architecture
- **Location**: Code comments + API_README.md
- **Tasks**:
  - Update code comments reflecting new architecture
  - Document migration strategy for future reference
  - Update API_README.md with CLI integration notes
  - Clean up legacy code comments

## Implementation Strategy

### Phase 3A: Foundation (P3-01, P3-02)
1. Create CLI adapter with full argument mapping
2. Implement protected icon logic preservation
3. Add basic integration tests

### Phase 3B: Integration (P3-03, P3-04)
1. Update main.rs to use new pipeline
2. Integrate theme system with new API
3. Test full CLI compatibility

### Phase 3C: Validation (P3-05, P3-06, P3-07)
1. Add comprehensive regression test suite
2. Validate performance characteristics
3. Update documentation and cleanup

## Critical Success Criteria

### Functional Requirements
- [ ] All CLI flags produce identical output to current system
- [ ] Protected icon logic works exactly as before
- [ ] Theme integration preserves all current behavior
- [ ] Error messages and exit codes remain identical
- [ ] Unicode/emoji handling works perfectly

### Technical Requirements
- [ ] No performance regression >5%
- [ ] All existing tests continue passing
- [ ] New regression tests cover edge cases
- [ ] Code maintains RSB compliance patterns

### Risk Mitigation
- [ ] Icon spacing validated in Windows Terminal
- [ ] Theme inheritance behavior preserved exactly
- [ ] Golden output tests prevent behavioral drift
- [ ] Rollback plan documented if issues found

## Testing Strategy

### Regression Test Categories
1. **CLI Flag Combinations**: Test all permutations
2. **Icon Logic**: Protected spacing and positioning
3. **Theme Integration**: All theme features work
4. **Unicode/Emoji**: Width calculations preserved
5. **Performance**: No degradation in critical paths

### Golden Output Tests
- Capture current CLI output for 50+ test cases
- Compare new system output byte-for-byte
- Include edge cases like empty input, long text, complex themes

## Dependencies

### Must Complete Before Starting
- [x] CHINA-05A Phase 2 completed (API surface parity)
- [x] All current tests passing (132 tests)
- [x] Protected icon logic documented and understood

### Parallel Work Possible
- Documentation updates can happen during implementation
- Performance benchmarking can run alongside development
- Theme integration can be tested independently

## Rollback Plan

If critical issues discovered:
1. Revert main.rs changes (single commit)
2. Keep new API system for library users
3. Delay CLI switchover to Phase 4
4. Document lessons learned for future attempt

## Success Metrics

- **100% CLI compatibility**: No behavioral changes
- **0 regression test failures**: All edge cases covered
- **<5% performance impact**: No significant slowdown
- **Protected logic preserved**: Icon spacing works perfectly
- **Clean architecture**: Legacy code removed, RSB compliance maintained