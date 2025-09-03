#!/usr/bin/env bash
#
# UAT Ceremonial Extensions - User-Driven Test Ceremonies
# Divine visual patterns for user acceptance testing
#
# portable: bash, boxy
# builtins: printf, test, local, read
#

# Source core UX kit
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/ux-kit.sh"

# ═══════════════════════════════════════════════════════════════════════════════
# UAT-SPECIFIC CEREMONIES
# ═══════════════════════════════════════════════════════════════════════════════

# UAT suite boundaries
uat_suite_start() {
    local suite_name="$1"
    local description="${2:-User Acceptance Testing}"
    local version="${3:-v1.0}"
    
    if has_boxy; then
        {
            echo "$suite_name"
            echo
            echo "$description"
            echo
            echo "Instructions:"
            echo "• Follow each numbered step in sequence"
            echo "• Verify expected results match actual output"
            echo "• Report any discrepancies by test step number"
            echo "• Press ENTER after each verification"
        } | boxy --theme info --title "🎭 UAT SUITE START" --footer "$version" --width max --layout dt
    else
        echo "╔═══════════════════════════════════════════════════════════════════════════════╗"
        echo "║                           UAT SUITE START                                    ║"  
        echo "║                          $suite_name                                         ║"
        echo "║                                                                               ║"
        echo "║ $description                                                                  ║"
        echo "╚═══════════════════════════════════════════════════════════════════════════════╝"
    fi
    echo
}

uat_suite_end() {
    local suite_name="$1"
    local total_steps="${2:-0}"
    local completion_time="${3:-unknown}"
    
    if has_boxy; then
        {
            echo "SUITE COMPLETED"
            echo
            echo "Suite: $suite_name"
            echo "Steps Executed: $total_steps"
            echo
            echo "If all steps passed visual verification,"
            echo "this feature is certified for user acceptance."
            echo
            echo "Thank you for your validation!"
        } | boxy --theme success --title "🏁 UAT SUITE COMPLETE" --footer "Duration: $completion_time" --width max --layout dt
    else
        echo "╔═══════════════════════════════════════════════════════════════════════════════╗"
        echo "║                           UAT SUITE COMPLETE                                 ║"
        echo "║                                                                               ║"
        echo "║ Suite: $suite_name                                                           ║"
        echo "║ Steps: $total_steps                                                          ║"
        echo "╚═══════════════════════════════════════════════════════════════════════════════╝"
    fi
    echo
}

# Step setup and assumptions box
uat_step_setup() {
    local step_num="$1"
    local feature="$2"
    local assumptions="$3"
    local setup_commands="${4:-}"
    
    if has_boxy; then
        local content="🔧 SETUP & ASSUMPTIONS
────────────────────────────────────────
Feature: $feature
Assumptions:
$assumptions"

        [[ -n "$setup_commands" ]] && {
            content="$content

Pre-requisites:
$setup_commands"
        }
        
        echo "$content" | boxy --theme warn --title "📋 STEP $step_num PREPARATION"
    else
        echo "┌─────────────────────────────────────────────────────────────────────────┐"
        echo "│ 📋 STEP $step_num PREPARATION                                            │"
        echo "├─────────────────────────────────────────────────────────────────────────┤"
        echo "│ Feature: $feature                                                       │"
        echo "│ Assumptions: $assumptions                                               │"
        [[ -n "$setup_commands" ]] && echo "│ Pre-requisites: $setup_commands                                         │"
        echo "└─────────────────────────────────────────────────────────────────────────┘"
    fi
    echo
}

# Command execution display
uat_command_display() {
    local step_num="$1"
    local command="$2"
    local description="${3:-Executing command}"
    
    if has_boxy; then
        cat <<-EOF | boxy --theme info --title "⚡ STEP $step_num EXECUTION"
		$description
		
		Command:
		$ $command
		EOF
    else
        echo "┌─────────────────────────────────────────────────────────────────────────┐"
        echo "│ ⚡ STEP $step_num EXECUTION                                               │"
        echo "├─────────────────────────────────────────────────────────────────────────┤"
        echo "│ $ $command                                                              │"
        echo "└─────────────────────────────────────────────────────────────────────────┘"
    fi
    echo
}

# Result display box
uat_result_display() {
    local step_num="$1"
    local result_data="$2"
    local result_file="${3:-}"
    
    if has_boxy; then
        local content="📤 ACTUAL RESULT:
────────────────────────────────────────
$result_data"
        
        [[ -n "$result_file" ]] && {
            content="$content

💾 Also saved to: $result_file"
        }
        
        echo "$content" | boxy --theme success --title "📊 STEP $step_num RESULT"
    else
        echo "┌─────────────────────────────────────────────────────────────────────────┐"
        echo "│ 📊 STEP $step_num RESULT                                                 │"
        echo "├─────────────────────────────────────────────────────────────────────────┤"
        echo "│ $result_data                                                            │"
        [[ -n "$result_file" ]] && echo "│ Saved to: $result_file                                                  │"
        echo "└─────────────────────────────────────────────────────────────────────────┘"
    fi
    echo
}

# Expectation box
uat_expectation_display() {
    local step_num="$1"
    local expectation="$2"
    local verification_points="${3:-}"
    
    if has_boxy; then
        local content="🎯 EXPECTED RESULT:
────────────────────────────────────────
$expectation"
        
        [[ -n "$verification_points" ]] && {
            content="$content

✓ Verification Points:
$verification_points"
        }
        
        echo "$content" | boxy --theme warn --title "🔍 STEP $step_num EXPECTATION"
    else
        echo "┌─────────────────────────────────────────────────────────────────────────┐"
        echo "│ 🔍 STEP $step_num EXPECTATION                                            │"
        echo "├─────────────────────────────────────────────────────────────────────────┤"
        echo "│ Expected: $expectation                                                  │"
        [[ -n "$verification_points" ]] && echo "│ Verify: $verification_points                                            │"
        echo "└─────────────────────────────────────────────────────────────────────────┘"
    fi
    echo
}

# Mental separation ceremony between steps
uat_step_separator() {
    local completed_step="$1"
    local next_step="${2:-}"
    
    if has_boxy; then
        local content="Step $completed_step completed"
        [[ -n "$next_step" ]] && content="$content → Proceeding to Step $next_step"
        echo "$content" | boxy --theme info --width max
    else
        echo
        echo "    ✓ Step $completed_step completed"
        [[ -n "$next_step" ]] && echo "    → Proceeding to Step $next_step"
        echo
    fi
    # If boxy is present, no need for manual separator line
    if ! has_boxy; then
        # Visual separator for non-boxy environments
        if has_color; then
            printf "${COLOR_CYAN}"
            printf '%.0s═' $(seq 1 80)
            printf "${COLOR_RESET}\n\n"
        else
            printf '%.0s═' $(seq 1 80)
            printf '\n\n'
        fi
    fi
}

# User verification pause
uat_user_verify() {
    local step_num="$1"
    local auto_mode="${UAT_AUTO:-false}"
    
    if [[ "$auto_mode" == "true" ]]; then
        # Auto mode - brief pause
        sleep 1
        echo "  [AUTO MODE] Step $step_num auto-verified"
        return 0
    fi
    
    # Interactive mode
    if has_boxy; then
        echo "Press ENTER to confirm Step $step_num verification, or 'q' to quit:" | \
            boxy --theme warn --width 60
    else
        echo "Press ENTER to confirm Step $step_num verification, or 'q' to quit:"
    fi
    
    read -r response
    if [[ "${response,,}" == "q" ]]; then
        echo "UAT terminated by user at Step $step_num"
        exit 1
    fi
}

# Complete UAT step orchestration
uat_execute_step() {
    local step_num="$1"
    local feature="$2"
    local assumptions="$3"
    local command="$4"
    local expectation="$5"
    local description="${6:-Executing test step}"
    local verification_points="${7:-}"
    local setup_commands="${8:-}"
    
    # Step setup
    uat_step_setup "$step_num" "$feature" "$assumptions" "$setup_commands"
    
    # Command display
    uat_command_display "$step_num" "$command" "$description"
    
    # Execute command and capture result
    local result_file="/tmp/uat_step_${step_num}_result.txt"
    local result_data
    
    if result_data=$(eval "$command" 2>&1); then
        echo "$result_data" > "$result_file"
        uat_result_display "$step_num" "$result_data" "$result_file"
    else
        local exit_code=$?
        result_data="Command failed with exit code: $exit_code\n$result_data"
        echo "$result_data" > "$result_file"
        uat_result_display "$step_num" "$result_data" "$result_file"
    fi
    
    # Expectation display
    uat_expectation_display "$step_num" "$expectation" "$verification_points"
    
    # User verification
    uat_user_verify "$step_num"
    
    # Return result for potential chaining
    echo "$result_data"
}

# Workflow progression ceremony
uat_workflow_start() {
    local workflow_name="$1"
    local workflow_description="$2"
    
    if has_boxy; then
        cat <<-EOF | boxy --theme info --title "🔄 WORKFLOW TESTING"
		Starting Workflow: $workflow_name
		
		Description: $workflow_description
		
		This workflow will test the complete user journey
		through this feature area.
		EOF
    else
        echo "═══ WORKFLOW TESTING ═══"
        echo "Workflow: $workflow_name"
        echo "Description: $workflow_description"
        echo "═══════════════════════"
    fi
    echo
}

uat_workflow_end() {
    local workflow_name="$1"
    local steps_completed="$2"
    
    if has_boxy; then
        cat <<-EOF | boxy --theme success --title "✅ WORKFLOW COMPLETE"
		Workflow Completed: $workflow_name
		
		Steps Executed: $steps_completed
		
		This workflow has been validated through
		complete user journey testing.
		EOF
    else
        echo "✅ WORKFLOW COMPLETE: $workflow_name"
        echo "Steps: $steps_completed"
    fi
    echo
}

# Export UAT functions
export -f uat_suite_start uat_suite_end
export -f uat_step_setup uat_command_display uat_result_display uat_expectation_display
export -f uat_step_separator uat_user_verify uat_execute_step
export -f uat_workflow_start uat_workflow_end
