#!/bin/bash
# ===============================================
# CEREMONY_07: LAYOUT CONTROL VALIDATION
# ===============================================
# Tests --layout positioning system
# Validates header/footer alignment: hr,fc,sl patterns
# Part of batch_02_intermediate API complexity

# Import Lucas's UAT ceremony framework
source "$(dirname "$0")/../../misc/uat-ceremonies.sh"

uat_ceremony_start "CEREMONY_07: Layout Control Validation"

# ===============================================
# STEP 1: Header Right, Footer Center Layout
# ===============================================
uat_execute_step "Header-Right, Footer-Center (--layout hr,fc)" '
echo "Layout positioning test" | "$BOXY_BIN" --header "Header Text" --footer "Footer Text" --layout hr,fc
' "Should display header right-aligned, footer center-aligned"

# ===============================================  
# STEP 2: Header Left, Footer Right Layout
# ===============================================
uat_execute_step "Header-Left, Footer-Right (--layout hl,fr)" '
echo "Alignment variation test" | "$BOXY_BIN" --header "Left Header" --footer "Right Footer" --layout hl,fr
' "Should display header left-aligned, footer right-aligned"

# ===============================================
# STEP 3: All Elements Center Layout
# ===============================================
uat_execute_step "All Center Layout (--layout hc,fc,sc)" '
echo "Centered content test" | "$BOXY_BIN" --header "Center Header" --footer "Center Footer" --status "Center Status" --layout hc,fc,sc
' "Should display all elements center-aligned"

# ===============================================
# STEP 4: Complex Multi-Element Layout
# ===============================================
uat_execute_step "Complex Layout (--layout hl,fr,sl,bc)" '
echo "Multi-alignment test" | "$BOXY_BIN" --title "Body Center" --header "Header Left" --footer "Footer Right" --status "Status Left" --layout hl,fr,sl,bc
' "Should apply different alignments to each element"

# ===============================================
# STEP 5: Status Bar Alignment Variations
# ===============================================
uat_execute_step "Status Right Alignment (--layout sr)" '
echo "Status positioning test" | "$BOXY_BIN" --status "Right Aligned Status" --layout sr
' "Should display status bar right-aligned"

# ===============================================
# STEP 6: Body Content Alignment
# ===============================================
uat_execute_step "Body Right Alignment (--layout br)" '
echo -e "Line 1 content\nLine 2 content\nLine 3 content" | "$BOXY_BIN" --layout br
' "Should display body content right-aligned"

# ===============================================
# STEP 7: Layout with Theme Integration
# ===============================================
uat_execute_step "Layout with Theme (--theme success --layout hr,fl)" '
echo "Theme plus layout test" | "$BOXY_BIN" --theme success --header "Success Header" --footer "Success Footer" --layout hr,fl
' "Should combine success theme styling with header-right, footer-left"

# ===============================================
# STEP 8: Divider Layout Options
# ===============================================
uat_execute_step "Layout with Dividers (--layout dt,ds)" '
echo "Divider layout test" | "$BOXY_BIN" --title "Title Above" --status "Status Below" --layout dt,ds
' "Should display dividers after title and before status"

# uat_ceremony_complete - function not defined