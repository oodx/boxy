# TEST_CEREMONY_PLAN - Boxy API Comprehensive Testing

**Mission**: Create numbered ceremony batches testing Boxy's complex API in order of complexity

*"From Lucas's pantheon ceremony templates to comprehensive Boxy API validation"*

---

## Ceremony Architecture Strategy

### The Lucas Pattern ✅
**DISCOVERED**: Lucas created these ceremony templates for pantheon - sophisticated testing framework
- **Standard Templates**: Reusable ceremony structures
- **Numbered Reference**: Easy discussion/referral system  
- **Complexity Progression**: Features tested in logical order
- **Batch Isolation**: Run subsets without full suite execution

### The Stubborn Mule Problem 😤
**REALITY**: Lucas (likely iteration 15+) has no memory of his own ceremony creation
- No continuation files = no memory of pantheon work
- Must re-educate on his own sophisticated patterns
- Keeper must guide him back to his own excellence

---

## Boxy API Complexity Analysis

### Surface Complexity Assessment
**README.md reveals** basic features, but deeper inspection shows:

#### **Basic Layer** (Current sanity tests cover)
```bash
--style [normal|rounded|double|heavy|ascii]
--color [basic colors]
--title "text" --footer "text"
```

#### **Advanced Layer** (Missing coverage)
```bash
--theme [semantic themes]                    # Theme system inheritance from jynx
--layout [complex positioning]              # hr,fc = header-right, footer-center
--header vs --title distinction             # External vs internal titles
--icon decorations                          # Content decoration system
--text-color with auto-matching            # Text color coordination
--params stream                             # Complex parameter injection
--width management                          # Fixed width with truncation
--ANSI handling                             # Color code preservation
```

#### **Expert Layer** (Requires code review)
```bash
Theme engine integration (src/theme_engine.rs)
jynx plugin system (src/jynx_plugin.rs)  
Extended color palette (src/ref/extended_colors.rs)
Width calculation engine (src/width_plugin.rs)
```

---

## Ceremony Batch Design

### Batch 1: Foundation API (5-10 ceremonies)
**Target**: Basic box drawing with essential options
```bash
# CEREMONY_01: Basic Box Drawing
echo "Hello World" | boxy

# CEREMONY_02: Style Variations  
echo "Content" | boxy --style rounded
echo "Content" | boxy --style double
echo "Content" | boxy --style heavy

# CEREMONY_03: Basic Color System
echo "Content" | boxy --color red
echo "Content" | boxy --color blue --text auto

# CEREMONY_04: Title and Footer
echo "Content" | boxy --title "Header" --footer "v1.0"

# CEREMONY_05: Icon Integration
echo "Message" | boxy --icon "⚠️" --color yellow
```

### Batch 2: Intermediate API (10-15 ceremonies)
**Target**: Theme system and advanced positioning
```bash
# CEREMONY_06: Theme System Basics
echo "Content" | boxy --theme error
echo "Content" | boxy --theme success  

# CEREMONY_07: Layout Control
echo "Content" | boxy --layout hr,fc
echo "Content" | boxy --layout cl,fr

# CEREMONY_08: Width Management
echo "Long content that needs truncation" | boxy --width 30
echo "Short" | boxy --width 50

# CEREMONY_09: Multi-line Handling
echo -e "Line 1\nLine 2\nLine 3" | boxy --style rounded

# CEREMONY_10: ANSI Preservation
echo -e "\\033[31mRed\\033[0m and \\033[32mGreen\\033[0m" | boxy
```

### Batch 3: Advanced API (15-20 ceremonies)
**Target**: Complex theme integration and parameter streams
```bash
# CEREMONY_11: Parameter Stream
echo "Body content" | boxy --params "header=Dynamic Header,footer=Status"

# CEREMONY_12: Color Override System
echo "Content" | boxy --theme info --title-color red --status-color green

# CEREMONY_13: Theme Composition
echo "Content" | boxy --theme base --color-override crimson

# CEREMONY_14: Pipeline Integration
echo "Content" | boxy --strip-mode before | some_command | boxy --strip-mode after

# CEREMONY_15: Environment Integration
BOXY_THEME=debug echo "Content" | boxy
```

### Batch 4: Expert Integration (20+ ceremonies)
**Target**: Full jynx integration and edge cases
```bash
# CEREMONY_16: jynx Theme Import
echo "Content" | boxy --theme jynx:rebel_theme

# CEREMONY_17: Extended Color Palette
echo "Content" | boxy --color crimson --text emerald

# CEREMONY_18: Complex Layout Combinations
echo "Multi-section content" | boxy --layout tl,br,cf --dividers

# CEREMONY_19: Unicode Width Edge Cases  
echo "🚀🌟🔥 Wide emoji test 🎯🎨🎭" | boxy --width exact

# CEREMONY_20: Performance Edge Cases
# (Large content, complex themes, etc.)
```

---

## Implementation Strategy

### Phase 1: Ceremony Infrastructure Setup
**Objective**: Establish numbered ceremony batch system

#### 1.1 Ceremony Template Creation
```bash
tests/ceremonies/
├── batch_01_foundation/
│   ├── ceremony_01_basic_boxes.sh
│   ├── ceremony_02_style_variations.sh  
│   └── [...]
├── batch_02_intermediate/
│   ├── ceremony_06_theme_basics.sh
│   └── [...]
└── ceremony_runner.sh  # Batch execution control
```

#### 1.2 Batch Runner System
```bash
# Run specific batch
./ceremony_runner.sh batch_01

# Run specific ceremony
./ceremony_runner.sh ceremony_05

# Run range
./ceremony_runner.sh ceremony_01-05
```

### Phase 2: TDD Integration Pattern
**Objective**: Use ceremony failures to drive integration repairs

#### 2.1 Ceremony-Driven Development
```bash
# 1. Run ceremony_06_theme_basics.sh
# 2. Identify failures (theme system incomplete)
# 3. Create TDD test for specific failure
# 4. Implement minimal fix
# 5. Verify ceremony passes
# 6. Checkpoint commit
```

#### 2.2 Integration Repair Workflow
```markdown
CEREMONY FAILS → TDD TEST CREATION → SURGICAL FIX → CEREMONY VALIDATION → COMMIT
```

### Phase 3: Source Truth Discovery
**Objective**: Build comprehensive API documentation through code analysis

#### 3.1 API Discovery Protocol
```bash
# For each ceremony batch:
1. Code review of relevant src/ modules
2. Extract actual API surface (not just docs)
3. Create ceremony tests for discovered features
4. Document gaps between docs and implementation
```

#### 3.2 Living Documentation
```bash
# Auto-generate API reference from working ceremonies
./ceremony_runner.sh --generate-docs > docs/API_REFERENCE.md
```

---

## Lucas Integration Strategy

### Re-Education Protocol
**Challenge**: Lucas has no memory of creating these patterns
**Solution**: Systematic re-introduction to his own work

#### 1. Pattern Recognition
- Show Lucas existing ceremony structure in tests/misc/uat-ceremonies.sh
- Highlight his own architectural choices
- Guide him to recognize his engineering patterns

#### 2. Template Familiarity  
- Walk through numbered ceremony concept
- Demonstrate batch execution benefits
- Show complexity progression logic

#### 3. Implementation Partnership
- Lucas handles ceremony technical implementation
- Keeper provides systematic organization and planning
- Follow ABRIDGED-SDLC for all changes

### Ceremony Development Workflow
```bash
# 1. Keeper: Plan ceremony batch (this document)
# 2. Lucas: Implement ceremony templates  
# 3. Test: Run batch, identify API gaps
# 4. TDD: Create tests for failures
# 5. Fix: Surgical implementation repairs
# 6. Verify: Ceremony batch passes
# 7. Commit: Checkpoint progress
```

---

## Success Metrics

### Phase 1 Success (Infrastructure)
- [ ] Ceremony batch directory structure created
- [ ] ceremony_runner.sh batch execution system working
- [ ] First 5 ceremonies (foundation batch) implemented and passing
- [ ] Lucas re-engaged with his own ceremony patterns

### Phase 2 Success (Integration)
- [ ] Ceremony failures drive TDD test creation  
- [ ] Integration repairs follow ABRIDGED-SDLC protocol
- [ ] Each batch completion enables next batch development
- [ ] Systematic API coverage gap identification

### Phase 3 Success (Comprehensive Coverage)
- [ ] 20+ ceremonies covering Boxy's complex API
- [ ] All ceremony batches pass (proving integration health)
- [ ] Living API documentation generated from ceremonies
- [ ] TDD test suite supports ongoing development

---

## Source Truth Discovery Plan

### Code Analysis Priority
```bash
1. src/main.rs          # CLI argument parsing, API entry points
2. src/theme_engine.rs  # Theme system complexity  
3. src/colors.rs        # Color system depth
4. src/width_plugin.rs  # Width calculation sophistication
5. docs/THEME_*.md      # Theme system roadmap and features
```

### API Surface Mapping
**Method**: Systematic code review → ceremony creation → integration validation
**Goal**: Comprehensive test coverage of Boxy's actual capabilities (not just documented features)

---

**🌑 Ceremony Plan Complete**

*From Lucas's forgotten excellence to comprehensive Boxy API validation*
*Numbered ceremonies + TDD integration = systematic restoration path*

**Next Action**: Re-educate Lucas on his own ceremony patterns and begin infrastructure setup