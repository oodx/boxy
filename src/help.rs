

use crate::colors::*;
use crate::jynx_plugin::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");


/// Show comprehensive CLI help with examples and usage patterns
pub fn show_comprehensive_help(jynx: &JynxPlugin) {
    // Generate help content - if jynx is available, we'll enhance it
    let help_header = format!("{} {} - {}", NAME, VERSION, DESCRIPTION);
    
    if jynx.is_active() {
        // Use jynx to enhance help output
        jynx_println(&help_header, "help", jynx);
        println!();
    } else {
        // Standard output without jynx
        println!("{}", help_header);
        println!();
    }
    
    // =============== OVERVIEW ===============
    println!("{}OVERVIEW:{}", get_color_code("azure"), RESET);
    println!("  Boxy v0.9 introduces a comprehensive theme system with semantic formatting,");
    println!("  enhanced layout control, and 90+ colors for professional CLI output.");
    println!();
    
    // =============== BASIC USAGE ===============
    println!("{}BASIC USAGE:{}", get_color_code("emerald"), RESET);
    println!("  echo \"content\" | {} [OPTIONS]", NAME);
    println!("  command | {} --theme <theme>", NAME);
    println!("  {} theme <action> [args]        # Theme management", NAME);
    println!();
    
    // =============== CORE OPTIONS ===============
    println!("{}CORE OPTIONS:{}", get_color_code("amber"), RESET);
    println!("  {}Visual Styling:{}", get_color_code("cyan"), RESET);
    println!("    -s, --style <STYLE>        Border style: normal, rounded, double, heavy, ascii");
    println!("    -c, --color <COLOR>        Border color from 90+ palette (see --colors)");
    println!("    --text <COLOR>             Text color: 'auto' matches border, 'none' default");
    println!("    -w, --width <WIDTH|max|auto>  Set width: number, 'max' (terminal), or 'auto'");
    println!();
    
    println!("  {}Content & Layout:{}", get_color_code("cyan"), RESET);
    println!("    --header <TEXT>            External header (above the box)");
    println!("    --title <TEXT>             Title line (first in-box line; emoji-aware icon)");
    println!("    --footer <TEXT>            Footer text (inside bottom border)");
    println!("    --icon <ICON>              Add icon to content (deprecated - use --title)"); //??
    println!("    --status <TEXT>            Status line inside box (use sl:|sc:|sr: prefixes)");
    println!("    --layout <spec>            Align/divide/pad: hl|hc|hr, fl|fc|fr, sl|sc|sr, dt|dtn, ds|dsn, stn|ptn|psn|ssn, bl|bc|br, bp");
    println!("    --pad <a|b>               Blank line above (a) and/or below (b) the body");
    println!("    --title-color <COLOR>      Color for title line (overrides --text)");
    println!("    --status-color <COLOR>     Color for status line (overrides --text)");
    println!("    --header-color <COLOR>     Color for header line");
    println!("    --footer-color <COLOR>     Color for footer line");
    println!();
    
    println!("  {}Theme System:{}", get_color_code("cyan"), RESET);
    println!("    --theme <THEME>            Apply semantic theme (error, success, warning, info)");
    println!();
    
    println!("  {}Utility:{}", get_color_code("cyan"), RESET);
    println!("    --no-boxy[=strict]         Strip box decoration (strict removes all formatting)");
    println!("    --no-color                 Disable jynx integration and color output");
    println!("    width                      Show terminal width diagnostics");
    println!("    --params <stream>          Param stream: k='v'; pairs (hd, tl, st, ft, ic). Body comes from stdin");
    println!("    -h, --help                 Show this help message");
    println!("    --colors                   Preview all 90+ available colors");
    println!("    -v, --version              Show version information");
    println!();
    
    // =============== THEME SYSTEM ===============
    println!("{}THEME SYSTEM:{}", get_color_code("violet"), RESET);
    println!("  {}Built-in Themes:{}", get_color_code("cyan"), RESET);
    println!("    error      Crimson borders, error icon (❌), bold text");
    println!("    success    Emerald borders, success icon (✅), rounded style");
    println!("    warning    Amber borders, warning icon (⚠️), italic text"); 
    println!("    info       Azure borders, info icon (ℹ️), normal style");
    println!("    critical   Enhanced error theme with double borders");
    println!();
    
    println!("  {}Theme Management:{}", get_color_code("cyan"), RESET);
    println!("    {} theme list                List available themes", NAME);
    println!("    {} theme show <name>         Show theme details", NAME);
    println!("    {} theme create <name>       Create new theme interactively", NAME);
    println!("    {} theme import <file>       Import theme from YAML", NAME);
    println!("    {} theme export <name>       Export theme to YAML", NAME);
    println!("    {} theme edit <name>         Edit existing theme", NAME);
    println!("    Env: BOXY_THEME=<name>      Set default theme (overridden by --theme)");
    println!();
    
    // =============== NEW IN V0.6 =============== //TODO:CLEANUP
    println!("{}NEW IN v0.6:{}", get_color_code("orchid"), RESET);
    println!("  {}Header vs Title Distinction:{}", get_color_code("cyan"), RESET);
    println!("    --header     External headers (app names, system labels)");
    println!("    --title      Internal titles (status, with icon integration)");
    println!();
    
    println!("  {}Enhanced Icon Integration:{}", get_color_code("cyan"), RESET);
    println!("    --title \"📦 Status\"       Icon automatically spaced and aligned");
    println!("    Instead of: --icon 📦 --title \"Status\"");
    println!();
    
    println!("  {}Status Bar Alignment:{}", get_color_code("cyan"), RESET);
    println!("    --status \"sl:Left\"        Left-aligned status");
    println!("    --status \"sc:Center\"      Center-aligned status");
    println!("    --status \"sr:Right\"       Right-aligned status");
    println!();
    
    println!("  {}Rich Color Palette:{}", get_color_code("cyan"), RESET);
    println!("    90+ colors including: crimson, emerald, azure, amber, violet");
    println!("    Semantic colors: error, success, warning, info, critical");
    println!("    Use --colors to preview all available colors");
    println!();
    
    // =============== EXAMPLES ===============
    println!("{}EXAMPLES:{}", get_color_code("gold"), RESET);
    
    println!("  {}Basic Usage:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Hello World\" | {}                    # Simple box", NAME);
    println!("    echo \"Data\" | {} --style rounded --color azure", NAME);
    println!();
    
    println!("  {}Theme Examples:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Operation failed\" | {} --theme error", NAME);
    println!("    echo \"Backup complete\" | {} --theme success", NAME);
    println!("    echo \"API deprecated\" | {} --theme warning", NAME);
    println!("    echo \"Server status\" | {} --theme info", NAME);
    println!();
    
    println!("  {}Advanced Layout:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Content\" | {} --header \"🚀 MyApp v2.1\" --title \"✅ Online\"", NAME);
    println!("    echo \"Status\" | {} --header \"System\" --status \"sr:Updated $(date)\"", NAME);
    println!("    echo \"Data\" | {} --title \"📊 Analytics\" --footer \"© 2024\"", NAME);
    println!();
    
    println!("  {}CI/CD Integration:{}", get_color_code("cyan"), RESET);
    println!("    # Build status reporting");
    println!("    build_status | {} --theme success --header \"Build Pipeline\"", NAME);
    println!("    test_results | {} --theme error --status \"sc:$(date)\"", NAME);
    println!();
    
    println!("  {}Content Processing:{}", get_color_code("cyan"), RESET);
    println!("    echo \"Raw content\" | {} --width 40        # Fixed width", NAME);
    println!("    cat log.txt | {} --no-boxy                # Strip formatting", NAME);
    println!("    echo \"Content\" | {} | {} --no-boxy=strict  # Remove all ANSI", NAME, NAME);
    println!();
    
    // =============== MIGRATION ===============
    // println!("{}MIGRATION (v0.5 → v0.6):{}", get_color_code("rust"), RESET);
    // println!("  {} migrate-commands --interactive    Interactive migration guide", NAME);
    // println!("  {} migrate-commands --check \"cmd\"     Analyze existing command", NAME);
    // println!("  {} migrate-commands --examples       Before/after examples", NAME);
    // println!("  {} migrate-commands --guide          Comprehensive migration guide", NAME);
    // println!();
    
    // println!("  {}Common Migrations:{}", get_color_code("cyan"), RESET);
    // println!("    OLD: --icon ✅ --title \"Status\"     → NEW: --title \"✅ Status\"");
    // println!("    OLD: --color red --style heavy      → NEW: --theme error");
    // println!("    OLD: --title \"MyApp\"               → NEW: --header \"MyApp\" --title \"🟢 Ready\"");
    // println!();
    
    // =============== TIPS ===============
    println!("{}TIPS & BEST PRACTICES:{}", get_color_code("sage"), RESET);
    println!("  • Use semantic themes (--theme error) over manual styling for consistency");
    println!("  • Headers for app identity, titles for status/state information");
    println!("  • Status alignment prefixes (sl:, sc:, sr:) for professional layouts");
    println!("  • Variable expansion works in headers, titles, footers: --title \"Status: $USER\"");
    println!("  • Chain with other commands: git status | {} --theme info --header \"Git\"", NAME);
    println!("  • Use --width for consistent formatting in scripts and CI/CD");
    println!();
    
    // =============== MORE INFO ===============
    println!("{}MORE INFORMATION:{}", get_color_code("steel"), RESET);
    println!("  {} --colors                 Preview color palette", NAME);
    println!("  {} theme list               Show available themes", NAME);
    println!("  GitHub: https://github.com/qodeninja/boxy");
    println!("  Documentation: See THEME_SYSTEM_v0.6.md");
    println!();
    
    println!("{}Transform your CLI output with semantic themes and professional formatting!{}", get_color_code("emerald"), RESET);
}

/// Show practical usage examples for different scenarios
pub fn show_usage_examples() {
    println!("{} {} - Usage Examples", NAME, VERSION);
    println!();
    
    // =============== QUICK START ===============
    println!("{}QUICK START EXAMPLES:{}", get_color_code("emerald"), RESET);
    println!("  # Basic usage - simple box around content");
    println!("  echo \"Hello World\" | {}", NAME);
    println!();
    println!("  # Apply semantic theme - automatic colors and icons"); 
    println!("  echo \"Error occurred\" | {} --theme error", NAME);
    println!("  echo \"Task complete\" | {} --theme success", NAME);
    println!("  echo \"Warning: deprecated\" | {} --theme warning", NAME);
    println!();
    
    // =============== LAYOUT EXAMPLES ===============
    println!("{}LAYOUT & STYLING:{}", get_color_code("azure"), RESET);
    println!("  # Header (external) vs Title (internal) distinction");
    println!("  echo \"Ready\" | {} --header \"MyApp v2.1\" --title \"🟢 Online\"", NAME);
    println!();
    println!("  # Different border styles and colors");
    println!("  echo \"Data\" | {} --style rounded --color azure", NAME);
    println!("  echo \"Alert\" | {} --style heavy --color crimson", NAME);
    println!("  echo \"Code\" | {} --style ascii --color steel", NAME);
    println!();
    println!("  # Fixed width for consistent formatting");
    println!("  echo \"Long content here\" | {} --width 30", NAME);
    println!();
    
    // =============== STATUS BARS ===============
    println!("{}STATUS BAR ALIGNMENT:{}", get_color_code("amber"), RESET);
    println!("  # Left, center, right aligned status bars");
    println!("  echo \"Data\" | {} --status \"sl:Processing...\"", NAME);
    println!("  echo \"Data\" | {} --status \"sc:50% Complete\"", NAME);
    println!("  echo \"Data\" | {} --status \"sr:Updated $(date '+%%H:%%M')\"", NAME);
    println!();
    
    // =============== DEVELOPMENT WORKFLOW ===============
    println!("{}DEVELOPMENT WORKFLOW:{}", get_color_code("violet"), RESET);
    println!("  # Git status with themes");
    println!("  git status --short | {} --theme warning --header \"Git Status\"", NAME);
    println!();
    println!("  # Build results");
    println!("  if make build; then");
    println!("    echo \"Build successful\" | {} --theme success", NAME);
    println!("  else");
    println!("    echo \"Build failed\" | {} --theme error", NAME);
    println!("  fi");
    println!();
    println!("  # Test results with status");
    println!("  pytest --tb=short | {} --theme info --header \"Test Suite\" --status \"sc:$(date)\"", NAME);
    println!();
    
    // =============== SYSTEM ADMINISTRATION ===============
    println!("{}SYSTEM ADMINISTRATION:{}", get_color_code("steel"), RESET);
    println!("  # Service status monitoring");
    println!("  systemctl status nginx | {} --header \"Nginx Status\" --theme info", NAME);
    println!();
    println!("  # Log analysis with fixed width");
    println!("  tail -10 /var/log/syslog | {} --width 80 --header \"System Log\"", NAME);
    println!();
    println!("  # Resource usage alerts");
    println!("  echo \"CPU: 85%, Memory: 92%\" | {} --theme warning --title \"⚠️ High Usage\"", NAME);
    println!();
    
    // =============== CI/CD INTEGRATION ===============
    println!("{}CI/CD INTEGRATION:{}", get_color_code("orchid"), RESET);
    println!("  # Pipeline status reporting");
    println!("  echo \"All tests passed\" | {} --theme success --header \"CI Pipeline\" --footer \"Build #42\"", NAME);
    println!();
    println!("  # Deployment notifications");
    println!("  echo \"Deployed to production\" | {} --theme success --header \"🚀 Deployment\" --status \"sr:$(git rev-parse --short HEAD)\"", NAME);
    println!();
    println!("  # Security scan results");
    println!("  echo \"3 vulnerabilities found\" | {} --theme warning --title \"🔒 Security Scan\"", NAME);
    println!();
    
    // =============== DATA PROCESSING ===============
    println!("{}DATA PROCESSING:{}", get_color_code("sage"), RESET);
    println!("  # Processing status with progress");
    println!("  echo \"Processed 1,247 records\" | {} --theme info --title \"📊 Data Processing\" --status \"sc:85% complete\"", NAME);
    println!();
    println!("  # Database operations");
    println!("  echo \"Backup completed\" | {} --theme success --header \"Database Backup\" --footer \"Size: 2.4GB\"", NAME);
    println!();
    println!("  # API responses");
    println!("  curl -s api/health | {} --theme info --header \"API Health Check\"", NAME);
    println!();
    
    // =============== ADVANCED USAGE ===============
    println!("{}ADVANCED USAGE:{}", get_color_code("rust"), RESET);
    println!("  # Variable expansion in text");
    println!("  echo \"Welcome\" | {} --header \"System: $HOSTNAME\" --title \"User: $USER\" --status \"sr:$(date)\"", NAME);
    println!();
    println!("  # Chain with other commands");
    println!("  ps aux | grep nginx | {} --header \"Nginx Processes\" --theme info", NAME);
    println!();
    println!("  # Remove box formatting (useful for parsing)");
    println!("  echo \"Content with ANSI\" | {} --theme success | {} --no-boxy=strict", NAME, NAME);
    println!();
    
    // =============== THEME MANAGEMENT ===============
    println!("{}THEME MANAGEMENT:{}", get_color_code("coral"), RESET);
    println!("  # List available themes");
    println!("  {} theme list", NAME);
    println!();
    println!("  # Create custom theme");
    println!("  {} theme create my_project_theme", NAME);
    println!();
    println!("  # Import/export themes");
    println!("  {} theme export error > error_theme.yml", NAME);
    println!("  {} theme import ~/my_theme.yml", NAME);
    println!();
    
    // // =============== MIGRATION ===============
    // println!("{}MIGRATION FROM v0.5:{}", get_color_code("amber"), RESET);
    // println!("  # Get migration help for existing commands");
    // println!("  {} migrate-commands --check \"echo 'test' | boxy --icon ✅ --color green\"", NAME);
    // println!();
    // println!("  # Interactive migration assistant");
    // println!("  {} migrate-commands --interactive", NAME);
    // println!();
    
    // =============== TIPS ===============
    println!("{}PRO TIPS:{}", get_color_code("emerald"), RESET);
    println!("  • Combine themes with specific overrides: --theme info --width 60");
    println!("  • Use headers for app identity, titles for status");
    println!("  • Status prefixes (sl:, sc:, sr:) provide professional alignment");
    println!("  • Fixed widths ensure consistent formatting in logs and reports");
    println!("  • Themes are faster than manual color/style combinations");
    println!();
    
    println!("{}More help: {} --help | {} --colors | {} theme help{}", 
        get_color_code("steel"), NAME, NAME, NAME, RESET);
}
