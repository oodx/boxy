#!/bin/bash
# Document Reference Validation Script
# Validates all document references in the process system

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "🔍 Validating Boxy Process Documentation References..."
echo "Project Root: $PROJECT_ROOT"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

# Get current timestamp for age calculations
CURRENT_TIME=$(date +%s)
ONE_WEEK_AGO=$((CURRENT_TIME - 604800))  # 7 days in seconds
ONE_MONTH_AGO=$((CURRENT_TIME - 2592000)) # 30 days in seconds

# Function to check if file exists and check age
check_file() {
    local file="$1"
    local context="$2"
    local critical="${3:-false}"  # Third param indicates if file should be frequently updated

    if [[ -f "$file" ]]; then
        local file_time
        file_time=$(stat -c %Y "$file" 2>/dev/null || stat -f %m "$file" 2>/dev/null)
        local age_indicator=""

        if [[ -n "$file_time" ]]; then
            if [[ "$critical" == "critical" ]]; then
                # Critical files should be updated within a week
                if [[ $file_time -lt $ONE_WEEK_AGO ]]; then
                    age_indicator=" ${YELLOW}(>1 week old - consider updating)${NC}"
                    ((WARNINGS++))
                fi
            else
                # Regular files get warning after a month
                if [[ $file_time -lt $ONE_MONTH_AGO ]]; then
                    age_indicator=" ${YELLOW}(>1 month old)${NC}"
                    ((WARNINGS++))
                fi
            fi
        fi

        if [[ -n "$age_indicator" ]]; then
            echo -e "${GREEN}✅${NC} $file ${BLUE}($context)${NC}$age_indicator"
        fi
        return 0
    else
        echo -e "${RED}❌${NC} $file ${BLUE}($context)${NC} - FILE NOT FOUND"
        ((ERRORS++))
        return 1
    fi
}

# Function to check if directory exists
check_dir() {
    local dir="$1"
    local context="$2"

    if [[ ! -d "$dir" ]]; then
        echo -e "${RED}❌${NC} $dir/ ${BLUE}($context)${NC} - DIRECTORY NOT FOUND"
        ((ERRORS++))
        return 1
    fi
    return 0
}

# Function to check file references within documents
check_references_in_file() {
    local file="$1"
    local context="$2"

    if [[ ! -f "$file" ]]; then
        echo -e "${RED}❌${NC} Cannot check references in $file - file not found"
        ((ERRORS++))
        return 1
    fi

    # Don't print per-file messages unless there are issues

    # Extract file references (basic pattern matching)
    # Look for patterns like: docs/*, .eggs/*, bin/*, etc.
    local found_issues=0

    while IFS= read -r line; do
        # Skip empty lines and comments
        [[ -z "$line" || "$line" =~ ^[[:space:]]*# ]] && continue

        # Look for file references
        if [[ "$line" =~ docs/[^[:space:]]+\.(txt|md) ]] || \
           [[ "$line" =~ \.eggs/[^[:space:]]+\.(txt|md) ]] || \
           [[ "$line" =~ bin/[^[:space:]]+\.sh ]]; then

            # Extract the file path
            local ref_file
            ref_file=$(echo "$line" | grep -oE '(docs/[^[:space:]]+\.(txt|md)|\.eggs/[^[:space:]]+\.(txt|md)|bin/[^[:space:]]+\.sh)' | head -1)

            if [[ -n "$ref_file" ]]; then
                if [[ ! -f "$ref_file" && ! -d "$ref_file" ]]; then
                    echo -e "  ${RED}❌${NC} $ref_file - REFERENCED BUT NOT FOUND (in $file)"
                    ((ERRORS++))
                    ((ref_issues++))
                fi
            fi
        fi
    done < "$file"

    # Only show message if we actually checked references
    local checked_refs=false
    while IFS= read -r line; do
        if [[ "$line" =~ docs/[^[:space:]]+\.(txt|md) ]] || \
           [[ "$line" =~ \.eggs/[^[:space:]]+\.(txt|md) ]] || \
           [[ "$line" =~ bin/[^[:space:]]+\.sh ]]; then
            checked_refs=true
            break
        fi
    done < "$file"
}

# Check all files silently, only output problems
check_file "START.txt" "main entry point"
check_file "docs/procs/QUICK_REF.txt" "quick reference" "critical"

check_dir "docs/procs" "process directory"
check_file "docs/procs/PROCESS.txt" "master workflow"
check_file "docs/procs/CONTINUE.md" "session status" "critical"
check_file "docs/procs/SPRINT.txt" "current sprint" "critical"
check_file "docs/procs/ROADMAP.txt" "strategic overview"
check_file "docs/procs/TASKS.txt" "detailed tasks"
check_file "docs/procs/DONE.txt" "completed work"

check_dir "docs/ref" "reference directory"
check_file "docs/ref/CRITICAL_FUNCTIONS_PROTECTION.md" "width protection guide"
check_file "docs/ref/THEME_SYSTEM.md" "theme architecture"
check_file "docs/ref/HEIGHT_FEATURE.md" "height system"
check_file "docs/ref/WIDTH_CALCULATION.md" "unicode handling"
check_file "docs/ref/PUBLIC_API_STRAT.txt" "library API strategy"
check_file "docs/ref/YAML_TOML_MIGRATION.txt" "migration planning"
check_file "docs/ref/BOXY_LESSONS.md" "development lessons"
check_file "docs/ref/TOKEN_NAMESPACE_CONCEPT.md" "token concepts"
check_file "docs/ref/BOXY_HEIGHT_STRAT.txt" "height strategy"

check_dir ".eggs" "analysis directory"
check_file ".eggs/egg.GOLDEN.boxy-architecture-wisdom.txt" "consolidated wisdom"
check_file ".eggs/CHINA-TECHNICAL-DEBT-TICKETS.txt" "architecture debt"
check_file ".eggs/red_egg.2.technical-debt-task-tickets.txt" "testing debt"
check_file ".eggs/egg.2.critical-mvp-triage.txt" "MVP analysis"

check_file "bin/test.sh" "main test script"
check_file "bin/snap.sh" "performance snapshots"
check_dir "tests" "test directory"

# Check internal references
ref_issues=0
check_references_in_file "START.txt" "entry point references"
check_references_in_file "docs/procs/PROCESS.txt" "process references"
check_references_in_file "docs/procs/QUICK_REF.txt" "quick ref references"

# Only show status if there were issues
if [[ $ERRORS -gt 0 || $WARNINGS -gt 0 || $ref_issues -gt 0 ]]; then
    echo
    echo "=== ISSUES FOUND ==="
    if [[ $ref_issues -eq 0 ]]; then
        echo -e "${BLUE}Internal references: all valid${NC}"
    fi
fi

echo
echo "=== VALIDATION SUMMARY ==="
if [[ $ERRORS -eq 0 ]]; then
    echo -e "${GREEN}🎉 ALL VALIDATIONS PASSED!${NC}"
    echo -e "${GREEN}✅ Document structure is consistent${NC}"
    echo -e "${GREEN}✅ All references are valid${NC}"
    echo -e "${GREEN}✅ Self-hydrating system is intact${NC}"
    exit 0
else
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo -e "${RED}Errors found: $ERRORS${NC}"
    if [[ $WARNINGS -gt 0 ]]; then
        echo -e "${YELLOW}Warnings: $WARNINGS${NC}"
    fi
    echo
    echo "Please fix the missing files or update the references."
    exit 1
fi