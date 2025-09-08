#!/bin/bash
# ===============================================
# CEREMONY_09: MULTILINE HANDLING VALIDATION
# ===============================================
# Tests complex multiline content layouts
# Validates line alignment, wrapping, and formatting
# Part of batch_02_intermediate API complexity

# Import Lucas's UAT ceremony framework
source "$(dirname "$0")/../../misc/uat-ceremonies.sh"

uat_ceremony_start "CEREMONY_09: Multiline Handling Validation"

# ===============================================
# STEP 1: Basic Multiline Content
# ===============================================
uat_execute_step "Basic Multiline (echo -e)" '
echo -e "First line\nSecond line\nThird line" | "$BOXY_BIN" --style rounded
' "Should display three separate lines with rounded border"

# ===============================================  
# STEP 2: Mixed Length Lines
# ===============================================
uat_execute_step "Mixed Length Lines" '
echo -e "Short\nThis is a much longer line of text\nMedium length line\nX" | "$BOXY_BIN" --style double
' "Should handle lines of varying lengths gracefully"

# ===============================================
# STEP 3: Multiline with Width Constraint
# ===============================================
uat_execute_step "Multiline with Width (--width 25)" '
echo -e "Line one content\nLine two has more content than fits\nLine three\nLine four is also quite long" | "$BOXY_BIN" --width 25 --style heavy
' "Should constrain all lines to 25 character width"

# ===============================================
# STEP 4: Multiline with Alignment
# ===============================================
uat_execute_step "Multiline Right Aligned (--layout br)" '
echo -e "Right align line 1\nRight align line 2\nRight align line 3" | "$BOXY_BIN" --layout br
' "Should right-align all body content lines"

# ===============================================
# STEP 5: Empty Lines Handling
# ===============================================
uat_execute_step "Multiline with Empty Lines" '
echo -e "Line 1\n\nLine 3\n\nLine 5" | "$BOXY_BIN" --style ascii
' "Should preserve empty lines in multiline content"

# ===============================================
# STEP 6: Large Content Block
# ===============================================
uat_execute_step "Large Multiline Content Block" '
echo -e "Configuration Summary:\n• Database: PostgreSQL 14.2\n• Cache: Redis 6.2.1\n• Server: Nginx 1.20.1\n• Framework: Node.js 18.7.0\n• Environment: Production\n• Status: All services operational\n• Last updated: $(date)" | "$BOXY_BIN" --theme info --width 50
' "Should handle larger multiline content with theme styling"

# ===============================================
# STEP 7: Multiline with Headers and Footers
# ===============================================
uat_execute_step "Multiline with Header/Footer" '
echo -e "Process 1: Running\nProcess 2: Stopped\nProcess 3: Running\nProcess 4: Pending" | "$BOXY_BIN" --header "System Status" --footer "$(date)" --theme warning
' "Should combine multiline body with header and footer"

# ===============================================
# STEP 8: Unicode and Special Characters
# ===============================================
uat_execute_step "Multiline with Unicode" '
echo -e "🚀 Deployment Status\n✅ API Server: Online\n⚠️  Database: Maintenance\n❌ Cache: Offline\n📊 Metrics: Available" | "$BOXY_BIN" --style rounded --theme success
' "Should handle Unicode characters in multiline content"

# uat_ceremony_complete - function not defined