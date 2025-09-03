# Boxy v0.5 → v0.6 Migration Guide

## Overview

This guide helps you migrate from boxy v0.5.x to v0.6.0, which introduces a comprehensive theme system, enhanced CLI interface, and breaking changes for better semantic clarity.

**Migration Timeline:**
- ✅ v0.6.0: New features available, old syntax shows warnings
- 🔄 v0.6.x: Deprecation period - both syntaxes work
- 🚨 v0.7.0: Old deprecated syntax will be removed

**Quick Start:**
```bash
# Run the interactive migration assistant  
boxy migrate-commands --interactive

# Or analyze your existing commands
boxy migrate-commands --check "your existing command"
```

## Breaking Changes Summary

### 1. New --header Flag for External Headers

**BREAKING**: The distinction between external headers (application labels) and internal titles (status indicators) is now explicit.

```bash
# v0.5.x (OLD)
echo "data" | boxy --title "MyApp v1.0"

# v0.6.0 (NEW) 
echo "data" | boxy --header "MyApp v1.0" --title "✅ Ready"
```

**Migration Strategy:**
- Application names, system labels → use `--header`
- Status indicators, states → use `--title`

### 2. Enhanced --title with Icon Support

**BREAKING**: Icon positioning is now integrated directly into `--title` for consistent spacing.

```bash
# v0.5.x (OLD)
echo "Success" | boxy --icon ✅ --title "Operation Complete"

# v0.6.0 (NEW)
echo "Success" | boxy --title "✅ Operation Complete"
```

**Migration Strategy:**
- Combine separate `--icon` and `--title` flags
- Use `--title "icon text"` pattern
- Icons are auto-detected and properly formatted

### 3. Status Bar Alignment System

**NEW FEATURE**: Status bars now support alignment prefixes for better visual control.

```bash
# v0.5.x (acceptable but basic)
echo "Done" | boxy --status "Build completed successfully"

# v0.6.0 (enhanced control)
echo "Done" | boxy --status "sc:Build completed successfully"  # center
echo "Done" | boxy --status "sl:Build started"                # left  
echo "Done" | boxy --status "sr:© 2024 MyApp"                 # right
```

**Alignment Prefixes:**
- `sl:` - Left aligned
- `sc:` - Center aligned  
- `sr:` - Right aligned

### 4. Comprehensive Theme System

**NEW FEATURE**: Semantic themes replace manual color/icon combinations.

```bash
# v0.5.x (manual styling)
echo "Failed" | boxy --icon ❌ --color red --style heavy

# v0.6.0 (semantic themes)
echo "Failed" | boxy --theme error
```

**Available Themes:**
- `error` - Red with error icon
- `success` - Green with success icon  
- `warning` - Orange with warning icon
- `info` - Blue with info icon
- `critical` - Bold red with critical styling

## Migration Tools

### Built-in Migration Assistant

Boxy v0.6.0 includes built-in migration tools:

```bash
# Analyze existing commands
boxy migrate-commands --check "echo 'test' | boxy --icon 📦 --title Status"

# Interactive migration guide
boxy migrate-commands --interactive

# View migration examples
boxy migrate-commands --examples

# Comprehensive guide
boxy migrate-commands --guide

# Show breaking changes
boxy migrate-commands --v6-changes
```

### Command Analysis Examples

**Example 1: Icon + Title Combination**
```bash
# Command to analyze
boxy migrate-commands --check "echo 'Success' | boxy --icon ✅ --title 'Status'"

# Output:
# 🔄 Icon + Title Combination
# Consider using --title with embedded icon instead of separate flags
# OLD: --icon "✅" --title "Status"  
# NEW: --title "✅ Status"
```

**Example 2: Long Status Text**
```bash
# Command to analyze  
boxy migrate-commands --check "echo 'Done' | boxy --status 'This is a very long status message'"

# Output:
# 📍 Status Alignment
# Long status text should use alignment prefixes for better control
# OLD: --status "This is a very long status message"
# NEW: --status "sc:This is a very long status message"
```

## Automated Migration Tools

### Script-Based Migration

Create a migration helper script to automatically update your boxy commands:

```bash
#!/bin/bash
# migrate_boxy_v6.sh - Automated migration helper

migrate_file() {
    local file="$1"
    echo "Migrating: $file"
    
    # Create backup
    cp "$file" "${file}.backup"
    
    # Replace common patterns
    sed -i \
        -e 's/--icon \([^ ]*\) --title "\([^"]*\)"/--title "\1 \2"/g' \
        -e 's/--color red.*--style heavy/--theme error/g' \
        -e 's/--color green.*--icon ✅/--theme success/g' \
        -e 's/--color orange.*--icon ⚠️/--theme warning/g' \
        -e 's/--color blue.*--icon ℹ️/--theme info/g' \
        "$file"
    
    echo "✅ Migration complete. Backup saved as ${file}.backup"
}

# Usage: ./migrate_boxy_v6.sh script.sh
migrate_file "$1"
```

### Bulk Migration Script

```bash
#!/bin/bash
# bulk_migrate_boxy.sh - Migrate all scripts in directory

find /path/to/your/scripts -name "*.sh" -o -name "*.bash" | while read -r script; do
    if grep -q "boxy" "$script"; then
        echo "Found boxy usage in: $script"
        boxy migrate-commands --check "$(grep boxy "$script" | head -1)" 
        echo "---"
    fi
done
```

## Step-by-Step Migration Process

### Phase 1: Assess Current Usage

1. **Inventory your boxy commands:**
   ```bash
   # Find all boxy usage across your system
   grep -r "boxy" /path/to/your/scripts/ > boxy_usage.txt
   
   # Or scan common script locations
   find ~/bin ~/.local/bin /usr/local/bin -name "*.sh" -exec grep -l "boxy" {} \;
   ```

2. **Categorize usage patterns:**
   - Application headers vs status titles
   - Icon + title combinations
   - Long status messages
   - Color/style combinations that could use themes

3. **Priority Assessment:**
   ```bash
   # High priority: Critical scripts, CI/CD, monitoring
   # Medium priority: Development tools, utilities
   # Low priority: Personal scripts, experiments
   ```

### Phase 2: Update Commands

3. **Fix header vs title usage:**
   ```bash
   # Pattern: Long descriptive titles → headers
   # OLD: --title "Application Dashboard v2.1"
   # NEW: --header "Application Dashboard v2.1" --title "🟢 Online"
   ```

4. **Consolidate icons and titles:**
   ```bash  
   # Pattern: Separate icon and title → unified title
   # OLD: --icon 🔥 --title "Urgent"
   # NEW: --title "🔥 Urgent"
   ```

5. **Add status alignment:**
   ```bash
   # Pattern: Long status → aligned status
   # OLD: --status "Long status message here"  
   # NEW: --status "sc:Long status message here"
   ```

6. **Apply semantic themes:**
   ```bash
   # Pattern: Manual styling → themes
   # OLD: --icon ❌ --color red --style heavy
   # NEW: --theme error
   ```

### Phase 3: Test and Validate

7. **Create test environment:**
   ```bash
   # Create test directory
   mkdir boxy_v6_testing
   cd boxy_v6_testing
   
   # Copy scripts for testing
   cp ~/bin/my_script.sh test_script.sh
   ```

8. **Test updated commands systematically:**
   ```bash
   # Test basic functionality
   echo "Test content" | boxy --header "MyApp" --title "✅ Ready"
   
   # Test theme integration
   echo "Error message" | boxy --theme error
   echo "Success message" | boxy --theme success
   
   # Test status alignment
   echo "Data" | boxy --status "sc:Center aligned status"
   
   # Test combined features
   echo "Complex output" | boxy --header "App v2.1" --title "🟢 Online" --status "sr:$(date)" --theme info
   ```

9. **Validate output formatting:**
   - ✅ Check icon spacing and alignment
   - ✅ Verify status bar positioning  
   - ✅ Confirm theme colors and styling
   - ✅ Test terminal width responsiveness
   - ✅ Verify backward compatibility

10. **Performance testing:**
    ```bash
    # Test rendering performance with new themes
    time (for i in {1..100}; do echo "test $i" | boxy --theme info >/dev/null; done)
    
    # Compare with old syntax
    time (for i in {1..100}; do echo "test $i" | boxy --color blue >/dev/null; done)
    ```

## Common Migration Patterns

### Pattern 1: Application Output

```bash
# OLD v0.5.x
echo "Server started on port 8080" | boxy --title "MyWebApp" --color blue

# NEW v0.6.0
echo "Server started on port 8080" | boxy --header "MyWebApp" --title "🚀 Started" --theme info
```

### Pattern 2: Error Messages

```bash
# OLD v0.5.x  
echo "Database connection failed" | boxy --icon ❌ --color red --title "Error"

# NEW v0.6.0
echo "Database connection failed" | boxy --theme error
```

### Pattern 3: Success Notifications

```bash
# OLD v0.5.x
echo "Backup completed successfully" | boxy --icon ✅ --color green --title "Backup"

# NEW v0.6.0  
echo "Backup completed successfully" | boxy --theme success
```

### Pattern 4: Status with Metadata

```bash
# OLD v0.5.x
echo "Processing..." | boxy --title "System Status" --footer "Updated: $(date)"

# NEW v0.6.0
echo "Processing..." | boxy --header "System Status" --title "⚙️ Processing" --status "sr:Updated: $(date)"
```

## Backward Compatibility

### Deprecation Timeline

- **v0.6.0**: New features introduced, old syntax shows warnings
- **v0.6.x**: Deprecation period - both syntaxes work  
- **v0.7.0**: Deprecated syntax will be removed

### Warning System

v0.6.0 shows helpful warnings for deprecated patterns:

```bash
$ echo "test" | boxy --icon 📦 --title "Status"
⚠️  DEPRECATION WARNING: Using --icon with --title may cause layout conflicts.
       → Try: --title "📦 Status"

💡 MIGRATION TIP: Use 'boxy migrate-commands --help' for migration assistance
```

### Compatibility Mode

All v0.5.x commands continue to work in v0.6.0:

```bash
# This still works in v0.6.0
echo "content" | boxy --color blue --style rounded --title "Old Style"

# But this is preferred
echo "content" | boxy --theme info --header "App Name" --title "✅ Status"
```

## New Features Available

### Theme Management

```bash
# List available themes
boxy theme list

# Create custom theme
boxy theme create my_project

# Import/export themes
boxy theme export error > error_theme.yml
boxy theme import custom_theme.yml

# Edit existing theme  
boxy theme edit my_project
```

### Enhanced Color Palette

90+ colors now available including:

```bash
# Rich semantic colors
--color crimson    # Deep red
--color emerald    # Rich green  
--color azure      # Sky blue
--color amber      # Golden orange

# Extended palette
--color violet, coral, sage, steel, ruby, pearl, etc.
```

### Advanced Layout Control

```bash
# Multiple alignment options
--status "sl:Left aligned status"
--status "sc:Center aligned status"  
--status "sr:Right aligned status"

# Combined usage
echo "data" | boxy --header "MyApp" --title "✅ Ready" --status "sr:v1.2.3"
```

## Rollback Strategy

If you encounter issues during migration, here's how to safely rollback:

### Immediate Rollback

```bash
# If using automated migration scripts with backups
find /path/to/scripts -name "*.backup" | while read -r backup; do
    original="${backup%.backup}"
    echo "Restoring: $original"
    mv "$backup" "$original"
done
```

### Version Rollback

```bash
# Downgrade to v0.5.x if needed (if you have the old binary)
sudo mv /usr/local/bin/boxy /usr/local/bin/boxy-v0.6-backup
sudo mv /usr/local/bin/boxy-v0.5 /usr/local/bin/boxy

# Or reinstall v0.5.x
cargo install boxy --version="0.5.0" --force
```

### Gradual Migration Approach

```bash
# Keep both versions during transition
alias boxy-old='/usr/local/bin/boxy-v0.5'  
alias boxy-new='/usr/local/bin/boxy-v0.6'

# Test with new version before committing
echo "test" | boxy-new --theme info
echo "test" | boxy-old --color blue  # fallback
```

## Troubleshooting

### Common Issues

**Issue 1: Icon spacing looks wrong**
```bash
# Problem: Manual spacing in old icon logic
echo "text" | boxy --icon "📦 " --title "Status"  # Extra space

# Solution: Use unified title
echo "text" | boxy --title "📦 Status"  # Automatic spacing
```

**Issue 2: Status bar not aligned**  
```bash
# Problem: No alignment prefix
echo "text" | boxy --status "Long message here"

# Solution: Add alignment prefix
echo "text" | boxy --status "sc:Long message here" 
```

**Issue 3: Colors don't work**
```bash
# Problem: Typo in color name
echo "text" | boxy --color "blue-green"  # Invalid

# Solution: Use correct color name or theme
echo "text" | boxy --color "teal"        # Valid color
echo "text" | boxy --theme info          # Or use theme
```

### Getting Help

```bash
# Check available colors
boxy --colors

# Get migration assistance
boxy migrate-commands --interactive

# View updated help
boxy --help

# Analyze specific command
boxy migrate-commands --check "your command here"
```

## Benefits of Migration

### Improved Semantics
- Clear distinction between headers and titles
- Consistent icon handling
- Better alignment control

### Enhanced Usability  
- Themes reduce command complexity
- Rich color palette
- Professional status bars

### Better Integration
- Perfect for CI/CD systems
- Enhanced script readability
- Consistent visual branding

## Next Steps

1. **Start with the migration assistant:**
   ```bash
   boxy migrate-commands --interactive
   ```

2. **Update your most-used commands first**
   
3. **Test thoroughly in non-production environments**

4. **Gradually roll out to all scripts**

5. **Take advantage of new theme system:**
   ```bash
   boxy theme list  # See available themes
   ```

6. **Create custom themes for your applications:**
   ```bash
   boxy theme create my_app
   ```

The migration to v0.6.0 brings powerful new capabilities while maintaining backward compatibility. Take your time, use the built-in tools, and enjoy the enhanced functionality!

## Quick Reference Card

```bash
# === HEADER VS TITLE ===
--header "App Name"        # External header (above box)
--title "✅ Status"        # Internal title (in border)

# === ICON INTEGRATION ===  
--title "📦 Combined"      # Icon + text in title

# === STATUS ALIGNMENT ===
--status "sl:Left"         # Left aligned
--status "sc:Center"       # Center aligned  
--status "sr:Right"        # Right aligned

# === THEMES ===
--theme error             # Red error theme
--theme success           # Green success theme
--theme warning           # Orange warning theme
--theme info              # Blue info theme

# === MIGRATION TOOLS ===
boxy migrate-commands --interactive    # Step-by-step guide
boxy migrate-commands --check "cmd"    # Analyze command
boxy migrate-commands --examples       # See examples
```

This completes your migration to the powerful boxy v0.6.0 theme system!