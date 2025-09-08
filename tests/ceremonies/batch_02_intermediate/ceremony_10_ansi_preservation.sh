#!/bin/bash
# ===============================================
# CEREMONY_10: ANSI PRESERVATION VALIDATION
# ===============================================
# Tests ANSI color code passthrough and preservation
# Validates complex color content handling
# Part of batch_02_intermediate API complexity

# Import Lucas's UAT ceremony framework
source "$(dirname "$0")/../../misc/uat-ceremonies.sh"

uat_ceremony_start "CEREMONY_10: ANSI Preservation Validation"

# ===============================================
# STEP 1: Basic ANSI Color Preservation
# ===============================================
uat_execute_step "Basic ANSI Colors" '
echo -e "\033[31mRed text\033[0m and \033[32mGreen text\033[0m" | "$BOXY_BIN"
' "Should preserve red and green ANSI color codes in content"

# ===============================================  
# STEP 2: Complex ANSI Sequence Preservation
# ===============================================
uat_execute_step "Complex ANSI Sequences" '
echo -e "\033[1;31mBold Red\033[0m \033[4;32mUnderlined Green\033[0m \033[3;34mItalic Blue\033[0m" | "$BOXY_BIN" --style rounded
' "Should preserve bold, underline, and italic ANSI formatting"

# ===============================================
# STEP 3: 256-Color ANSI Support
# ===============================================
uat_execute_step "256-Color ANSI Support" '
echo -e "\033[38;5;196mBright Red\033[0m \033[38;5;46mBright Green\033[0m \033[38;5;21mBright Blue\033[0m" | "$BOXY_BIN" --style double
' "Should preserve 256-color ANSI escape sequences"

# ===============================================
# STEP 4: ANSI with Multiline Content
# ===============================================
uat_execute_step "ANSI Multiline Content" '
echo -e "\033[31mLine 1: Red\033[0m\n\033[32mLine 2: Green\033[0m\n\033[34mLine 3: Blue\033[0m" | "$BOXY_BIN" --width 30
' "Should preserve ANSI colors across multiple lines"

# ===============================================
# STEP 5: ANSI with Theme Integration
# ===============================================
uat_execute_step "ANSI with Theme" '
echo -e "Status: \033[32mONLINE\033[0m | Errors: \033[31m0\033[0m" | "$BOXY_BIN" --theme info
' "Should preserve content ANSI colors while applying theme styling"

# ===============================================
# STEP 6: Mixed Content and ANSI
# ===============================================
uat_execute_step "Mixed Content and ANSI" '
echo -e "Normal text \033[1;33mWarning!\033[0m More normal text \033[32m✓ OK\033[0m" | "$BOXY_BIN" --style heavy
' "Should handle mixed normal text and ANSI colored text"

# ===============================================
# STEP 7: ANSI Background Colors
# ===============================================
uat_execute_step "ANSI Background Colors" '
echo -e "\033[41;37mRed Background\033[0m \033[42;30mGreen Background\033[0m" | "$BOXY_BIN"
' "Should preserve ANSI background color codes"

# ===============================================
# STEP 8: Complex ANSI with Width Constraints
# ===============================================
uat_execute_step "ANSI with Width Limit" '
echo -e "\033[31mThis is a long red line that should be wrapped or truncated\033[0m \033[32mAnd this is green text following\033[0m" | "$BOXY_BIN" --width 25
' "Should handle ANSI sequences properly with width constraints"

# uat_ceremony_complete - function not defined