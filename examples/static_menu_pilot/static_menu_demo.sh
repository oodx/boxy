#!/bin/bash

# Static Menu Pilot Demo - Proof of Concept
# This demonstrates persistent UI elements with dynamic content updates

# Terminal control functions
setup_terminal() {
    # Hide cursor, enable alternate screen buffer
    tput civis
    tput smcup
    
    # Clear screen
    clear
    
    # Get terminal dimensions
    ROWS=$(tput lines)
    COLS=$(tput cols)
    
    # Calculate areas
    HEADER_ROW=0
    MENU_ROW=1
    CONTENT_START=3
    CONTENT_END=$((ROWS - 3))
    FOOTER_ROW=$((ROWS - 1))
}

cleanup_terminal() {
    # Restore cursor, exit alternate screen buffer
    tput cnorm
    tput rmcup
}

draw_static_elements() {
    # Draw header (static)
    tput cup $HEADER_ROW 0
    tput rev  # Reverse video
    printf "%-${COLS}s" "🤖 AI Assistant Demo - Static Menu System $(date +%H:%M:%S)"
    tput sgr0  # Reset formatting
    
    # Draw menu (static)
    tput cup $MENU_ROW 0
    tput bold
    printf "%-${COLS}s" "[C]hat [T]ask [S]earch [H]elp [Q]uit"
    tput sgr0
    
    # Draw separator
    tput cup 2 0
    printf "%*s" $COLS | tr ' ' '─'
    
    # Draw footer (static)
    tput cup $FOOTER_ROW 0
    tput rev
    printf "%-${COLS}s" "Ready - Press keys for demo | Ctrl+C to exit"
    tput sgr0
}

update_content_area() {
    local message="$1"
    local line_num="${2:-0}"
    
    # Move to content area and update ONLY this line
    tput cup $((CONTENT_START + line_num)) 2
    tput el  # Clear to end of line
    echo "$message"
}

simulate_chat() {
    update_content_area "🧑 User: Hello AI assistant!" 0
    sleep 1
    update_content_area "🤖 AI: Hello! I'm responding without redrawing everything!" 1
    sleep 1
    update_content_area "🧑 User: This is amazing - the header/footer stay put!" 2
    sleep 1  
    update_content_area "🤖 AI: Exactly! Only content updates. Watch this:" 3
    sleep 0.5
    
    # Animate some responses
    for i in {1..5}; do
        update_content_area "🤖 Thinking$(printf "%*s" $i | tr ' ' '.')    " 4
        sleep 0.3
    done
    
    update_content_area "🤖 AI: Perfect! Static UI + dynamic content = professional UX" 4
}

simulate_task_mode() {
    # Clear content area
    for i in $(seq 0 $((CONTENT_END - CONTENT_START))); do
        tput cup $((CONTENT_START + i)) 0
        tput el
    done
    
    update_content_area "📋 TASK MODE ACTIVATED" 0
    sleep 0.5
    update_content_area "• Analyzing request..." 1
    sleep 0.8
    update_content_area "• Creating todo list..." 2  
    sleep 0.8
    update_content_area "• ✅ Task 1: Build static menu demo" 3
    sleep 0.5
    update_content_area "• ✅ Task 2: Prove terminal control works" 4
    sleep 0.5
    update_content_area "• 🔄 Task 3: Show smooth updates" 5
    sleep 1
    update_content_area "• ✅ Task 3: Show smooth updates" 5
}

# Trap to cleanup on exit
trap cleanup_terminal EXIT

# Main demo
echo "🚀 Starting Static Menu Demo..."
echo "This will show persistent header/footer with dynamic content updates"
echo "Press Enter to begin..."
read

setup_terminal
draw_static_elements

# Demo loop
while true; do
    # Position cursor in content area for any user input
    tput cup $CONTENT_START 0
    
    # Show demo options
    update_content_area "Demo Options:" 0
    update_content_area "  c - Chat simulation" 1
    update_content_area "  t - Task mode simulation" 2 
    update_content_area "  r - Redraw static elements" 3
    update_content_area "  q - Quit demo" 4
    update_content_area "" 5
    update_content_area "Choose option: " 6
    
    # Position cursor for input
    tput cup $((CONTENT_START + 6)) 16
    
    read -n 1 -s key
    case $key in
        c|C)
            simulate_chat
            ;;
        t|T)  
            simulate_task_mode
            ;;
        r|R)
            draw_static_elements
            ;;
        q|Q)
            break
            ;;
        *)
            update_content_area "Unknown option: '$key' - try c, t, r, or q" 7
            ;;
    esac
    
    sleep 1
done

echo "Demo complete!"