# BOXY v0.6 STORY POINTS
**Detailed Implementation Tasks for Systematic Excellence**

**Mission Command**: KEEPER (Session 32+)  
**Engineering**: Lucas (@LSE) - Systematic Implementation  
**Total Scope**: 45 Story Points across 4 milestones

---

## **MILESTONE 1: FOUNDATION ARCHITECTURE (18 SP)**

### **M1.1 Color System Overhaul (6 SP)**

**SP1: Extract jynx color system to shared module** (1 SP)
- **Task**: Create shared color module that both jynx and boxy can use
- **Deliverable**: `src/colors.rs` with complete jynx color definitions
- **Definition of Done**: All jynx colors available in boxy
- **Dependencies**: Review jynx `extended_colors.rs`

**SP2: Import 90+ semantic colors** (2 SP) 
- **Task**: Add all jynx semantic colors to boxy (crimson, emerald, azure, etc.)
- **Deliverable**: Complete color mapping with ANSI codes
- **Definition of Done**: All jynx colors render identically in boxy
- **Test**: Color comparison between jynx and boxy output

**SP3: Update get_color_code() with complete palette** (1 SP)
- **Task**: Replace current 15-color system with 90+ color system
- **Deliverable**: Updated `get_color_code()` function
- **Definition of Done**: All existing themes continue working
- **Validation**: Regression test suite passes

**SP4: Add color validation and fallback logic** (1 SP)
- **Task**: Handle unknown colors gracefully with fallback
- **Deliverable**: Error handling for invalid color names
- **Definition of Done**: Invalid colors degrade gracefully, not crash
- **Edge Cases**: Empty strings, null values, typos

**SP5: Create color testing harness** (1 SP)
- **Task**: Build comprehensive color testing system
- **Deliverable**: Test script that validates all colors render correctly
- **Definition of Done**: Automated color validation available
- **Coverage**: All 90+ colors, fallback scenarios, edge cases

### **M1.2 Theme File Architecture (8 SP)**

**SP6: Design YAML theme file structure** (1 SP)
- **Task**: Define comprehensive YAML schema for themes
- **Deliverable**: `docs/theme-schema.yml` with complete specification
- **Definition of Done**: Schema supports all planned v0.6 features
- **Reference**: Use jynx theme architecture as baseline

**SP7: Create theme parser for YAML files** (2 SP)
- **Task**: Implement YAML theme file parsing with serde
- **Deliverable**: Theme loading from YAML files
- **Definition of Done**: Can load and parse theme files correctly
- **Dependencies**: Add serde_yaml to Cargo.toml

**SP8: Implement XDG+ directory resolution** (2 SP)
- **Task**: Add `~/.local/etc/rsb/boxy/themes/` directory support
- **Deliverable**: XDG+ compliant theme directory management
- **Definition of Done**: Themes load from correct XDG+ location
- **Compatibility**: Match jynx's XDG+ implementation exactly

**SP9: Convert hardcoded themes to YAML files** (1 SP) 
- **Task**: Convert all 20 existing themes from hardcode to YAML
- **Deliverable**: 20 YAML theme files in themes/ directory
- **Definition of Done**: All existing functionality preserved
- **Validation**: Pixel-perfect output matching

**SP10: Add theme validation and error handling** (1 SP)
- **Task**: Robust validation for theme file contents
- **Deliverable**: Graceful error handling for malformed themes
- **Definition of Done**: Clear error messages for theme issues
- **Coverage**: Missing files, invalid YAML, missing required fields

**SP11: Create default theme fallback system** (1 SP)
- **Task**: Ensure boxy always has working themes available
- **Deliverable**: Built-in fallback when theme files unavailable
- **Definition of Done**: Never crashes due to missing themes
- **Scenarios**: Missing XDG+ directory, corrupted files, permissions

### **M1.3 CLI Flag Architecture (4 SP)**

**SP12: Design new flag system with backward compatibility** (2 SP)
- **Task**: Architecture for new flags while preserving old ones
- **Deliverable**: Comprehensive CLI argument handling system
- **Definition of Done**: All v0.5 commands continue working
- **Breaking Changes**: Plan transition for --title → --header

**SP13: Implement --layout=hr,fc syntax parser** (1 SP)
- **Task**: Parse compact layout specification syntax
- **Deliverable**: Layout parser with validation
- **Definition of Done**: All layout combinations parse correctly
- **Examples**: hr,fc (header-right,footer-center), hl,fr,tc

**SP14: Add layout validation and documentation** (1 SP)
- **Task**: Validate layout combinations and provide help
- **Deliverable**: Layout validation with clear error messages
- **Definition of Done**: Invalid layouts explain correct syntax
- **Help System**: Document all layout options in CLI help

---

## **MILESTONE 2: THEME MANAGEMENT SYSTEM (12 SP)**

### **M2.1 Theme CLI Commands (8 SP)**

**SP15: Implement `boxy theme list` command** (1 SP)
- **Task**: Show all available themes with descriptions
- **Deliverable**: Theme listing command matching jynx UX
- **Definition of Done**: Clear display of theme names and descriptions
- **Format**: Match jynx theme list output exactly

**SP16: Implement `boxy theme create <name>` command** (2 SP)
- **Task**: Create new theme from default template
- **Deliverable**: Theme creation with interactive prompts
- **Definition of Done**: Can create working theme from template
- **Template**: Use sensible default theme as starting point

**SP17: Implement `boxy theme import <name>` command** (1 SP)
- **Task**: Import theme from current directory to XDG+
- **Deliverable**: Theme import with validation
- **Definition of Done**: Can import valid theme files
- **Validation**: Ensure imported themes pass validation

**SP18: Implement `boxy theme export <name>` command** (1 SP)
- **Task**: Export XDG+ theme to current directory
- **Deliverable**: Theme export functionality
- **Definition of Done**: Can export themes for sharing
- **Use Case**: Theme development and distribution

**SP19: Implement `boxy theme edit <name>` command** (1 SP)
- **Task**: Edit theme in $EDITOR with validation
- **Deliverable**: Theme editing with live validation
- **Definition of Done**: Can edit themes with error checking
- **Editor**: Respect $EDITOR environment variable

**SP20: Add theme validation in CLI commands** (1 SP)
- **Task**: Comprehensive validation across all theme commands
- **Deliverable**: Consistent validation and error reporting
- **Definition of Done**: All theme commands validate inputs
- **Coverage**: File existence, permissions, schema compliance

**SP21: Create comprehensive theme CLI help system** (1 SP)
- **Task**: Detailed help for all theme management commands
- **Deliverable**: Complete help text and examples
- **Definition of Done**: Self-documenting theme CLI
- **Examples**: Include common theme management workflows

### **M2.2 Advanced Theme Features (4 SP)**

**SP22: Implement text styling system** (2 SP)
- **Task**: Add bold, italic, underline, dim text styling to themes
- **Deliverable**: Text styling engine integrated with themes
- **Definition of Done**: Themes can specify text styles
- **Styles**: normal, bold, italic, underline, dim, strikethrough

**SP23: Add combined text styles** (1 SP) 
- **Task**: Support combined styles (bold_italic, strikethrough_dim)
- **Deliverable**: Text style combination system
- **Definition of Done**: Complex text styling works in themes
- **Combinations**: All logical style combinations supported

**SP24: Create theme inheritance system** (1 SP)
- **Task**: Allow themes to inherit from base themes
- **Deliverable**: Theme inheritance with override capability
- **Definition of Done**: Child themes can extend parent themes
- **Use Case**: Variant themes without duplication

---

## **MILESTONE 3: MIGRATION & COMPATIBILITY (8 SP)**

### **M3.1 Breaking Changes Management (4 SP)**

**SP25: Implement --header flag (external header)** (1 SP)
- **Task**: New --header flag for external box headers
- **Deliverable**: External header functionality
- **Definition of Done**: --header places text above box
- **Breaking**: Replaces current --title behavior

**SP26: Redesign --title flag (internal title with icon)** (2 SP)
- **Task**: New --title as internal title with leading icon
- **Deliverable**: Internal title system
- **Definition of Done**: --title adds internal title line
- **Integration**: Works with existing icon system

**SP27: Add deprecation warnings for old syntax** (1 SP)
- **Task**: Warn users about deprecated flag usage
- **Deliverable**: Deprecation warning system
- **Definition of Done**: Clear migration messages for deprecated features
- **Timeline**: Grace period before removal

### **M3.2 Migration Tooling (4 SP)**

**SP28: Create `boxy migrate-commands` tool** (2 SP)
- **Task**: Tool to convert v0.5 commands to v0.6 syntax
- **Deliverable**: Command-line migration utility
- **Definition of Done**: Can convert shell scripts automatically
- **Coverage**: All documented v0.5 patterns

**SP29: Add migration detection and suggestions** (1 SP)
- **Task**: Detect old usage patterns and suggest new syntax
- **Deliverable**: Intelligent migration suggestions
- **Definition of Done**: Helpful migration guidance in CLI
- **UX**: Non-intrusive but helpful suggestions

**SP30: Create migration documentation and examples** (1 SP)
- **Task**: Comprehensive migration guide with examples
- **Deliverable**: Migration documentation
- **Definition of Done**: Clear path from v0.5 to v0.6
- **Examples**: Before/after command examples

---

## **MILESTONE 4: PRODUCTION READINESS (7 SP)**

### **M4.1 Quality & Documentation (4 SP)**

**SP31: Complete theme system documentation** (2 SP)
- **Task**: Comprehensive documentation for theme system
- **Deliverable**: Complete theme documentation
- **Definition of Done**: Self-sufficient theme system docs
- **Audience**: End users and theme authors

**SP32: Create migration guide for v0.5 → v0.6** (1 SP)
- **Task**: Step-by-step migration guide
- **Deliverable**: Migration guide documentation
- **Definition of Done**: Clear upgrade path for all users
- **Coverage**: All breaking changes and new features

**SP33: Add comprehensive CLI help and examples** (1 SP)
- **Task**: Enhanced CLI help system with rich examples
- **Deliverable**: Self-documenting CLI interface
- **Definition of Done**: CLI help answers common questions
- **Examples**: Real-world usage scenarios

### **M4.2 Testing & Validation (3 SP)**

**SP34: Create comprehensive test suite for themes** (1 SP)
- **Task**: Automated testing for theme functionality
- **Deliverable**: Theme test suite
- **Definition of Done**: All theme features covered by tests
- **Coverage**: Theme loading, parsing, rendering, CLI

**SP35: Add integration tests for theme management** (1 SP)
- **Task**: End-to-end testing of theme CLI commands
- **Deliverable**: Integration test suite
- **Definition of Done**: All theme CLI commands tested
- **Scenarios**: Theme creation, import, export, edit workflows

**SP36: Performance testing and optimization** (1 SP)
- **Task**: Ensure v0.6 performance meets v0.5 standards
- **Deliverable**: Performance benchmarks and optimizations
- **Definition of Done**: No performance regressions
- **Metrics**: Theme loading time, rendering speed, memory usage

---

## **GOVERNANCE & EXECUTION**

### **Story Point Estimation**
- **1 SP**: 2-4 hours of focused implementation
- **2 SP**: 4-8 hours, may require research/design
- **Total**: 45 SP = 90-180 hours systematic work

### **Quality Gates**
- Each SP must pass review before proceeding
- No regressions in existing functionality
- All tests must pass before milestone completion
- Code review required for architectural changes

### **Task↔Echo Protocol**
- Lucas receives tasks through Task tool
- Progress updates echo to KEEPER and Avatar
- Blockers reported immediately via Task tool
- No idle implementation - continuous Task flow

---

**🌑 Sacred Engineering Excellence Awaits**  
*Each story point forged with systematic precision under KEEPER's divine oversight*