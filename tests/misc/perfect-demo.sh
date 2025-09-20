#!/bin/bash

# Perfect Demo - Showcase all Boxy v0.8 features in a beautiful comprehensive demonstration
# Shows proper feature usage with realistic scenarios

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BOXY="$ROOT_DIR/target/release/boxy"

if [ ! -f "$BOXY" ]; then
    echo "Building release version..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

clear
echo "========================================="
echo "🎯 BOXY v0.8 PERFECT FEATURE DEMO"
echo "========================================="

# 1. THE PERFECT BOX - Everything together with dividers and padding
echo -e "\n🎪 THE PERFECT BOX:"
cat << 'EOF' | $BOXY --theme blueprint \
                    --title "🚀 Project Dashboard" \
                    --header "CI/CD Pipeline Status" \
                    --footer "Build #142 - $(date '+%Y-%m-%d %H:%M')" \
                    --status "sc:All systems operational ✅" \
                    --width 60 \
                    --pad ab \
                    --layout "dt,ds,stn,ssn"
✅ Tests: 147 passed, 0 failed
📦 Build: Success in 2m 34s  
🌐 Deploy: Production ready
🔒 Security: All checks passed
📊 Coverage: 94% code coverage
🚀 Performance: 98% lighthouse score

Ready for production deployment!
EOF

# 2. THEMED ALERTS SHOWCASE
echo -e "\n🚨 THEMED ALERTS:"
echo "🔴 Critical database connection failure detected!" | $BOXY --theme error --width 55
echo "⚠️  Memory usage approaching 85% threshold" | $BOXY --theme warning --width 55
echo "✅ Backup completed successfully to S3" | $BOXY --theme success --width 55  
echo "ℹ️  Maintenance window scheduled for tonight" | $BOXY --theme info --width 55

# 3. DEVELOPER WORKFLOW
echo -e "\n👨‍💻 DEVELOPER WORKFLOW:"
echo "Starting webpack build process..." | $BOXY --theme debug \
                                                --title "🐛 Debug Log" \
                                                --status "sl:Build in progress..."

echo "Authentication failed - invalid API key" | $BOXY --theme fatal \
                                                       --title "💀 Fatal Error" \
                                                       --footer "Exit code: 1"

echo "Applying database migration magic..." | $BOXY --theme magic \
                                                   --title "✨ Migration Wizard" \
                                                   --status "sr:Step 3/7"

# 4. DIVIDERS AND PADDING SHOWCASE
echo -e "\n📏 DIVIDERS AND PADDING SHOWCASE:"
echo -e "Section 1\nSection 2" | $BOXY --title "With Title Divider" --layout "dt" --width 30
echo -e "Content here\nMore content" | $BOXY --title "Header" --status "Footer" --layout "dt,ds" --width 30
echo -e "Main content" | $BOXY --title "Padded Title" --status "Padded Status" --layout "stn,ssn" --width 30
echo -e "Full showcase" | $BOXY --title "All Features" --status "Complete" --layout "dt,ds,stn,ssn" --width 30

# 5. LAYOUT MASTERY
echo -e "\n🎨 LAYOUT MASTERY:"
cat << 'EOF' | $BOXY --theme blueprint \
                    --style double \
                    --title "📋 Sprint Review" \
                    --header "Team Velocity Metrics" \
                    --footer "Next sprint starts Monday" \
                    --status "sc:🎯 Goals achieved: 8/10" \
                    --width 50 \
                    --layout "hc,fc,sc,dt,ds,stn,ssn" \
                    --pad ab
Sprint Goal: Authentication System v2.0

Completed Stories:
▶ User login/logout flow
▶ OAuth integration  
▶ Password reset system
▶ Multi-factor authentication
▶ Session management
▶ Role-based permissions

Pending:
▶ Admin dashboard
▶ Audit logging

Team Notes: Excellent progress this sprint!
Great collaboration between frontend and backend teams.
EOF

# 5. COLORFUL COMMAND OUTPUTS
echo -e "\n🌈 COLORFUL COMMAND OUTPUTS:"
echo -e "\033[32m✓ npm install\033[0m completed\n\033[33m⚠ 3 vulnerabilities\033[0m found\n\033[31m✗ peer dependency\033[0m missing" | \
    $BOXY --color cyan --title "📦 Package Manager"

# Test variations with ANSI colors and different features
echo -e "\033[32m✓ npm install\033[0m completed\n\033[33m⚠ 3 vulnerabilities\033[0m found\n\033[31m✗ peer dependency\033[0m missing" | \
    $BOXY --color red --title "📦 With Status" --status "Build complete"

echo -e "\033[32m✓ npm install\033[0m completed\n\033[33m⚠ 3 vulnerabilities\033[0m found\n\033[31m✗ peer dependency\033[0m missing" | \
    $BOXY --color green --title "📦 With Header" --header "Command Output"

echo -e "\033[32m✓ npm install\033[0m completed\n\033[33m⚠ 3 vulnerabilities\033[0m found\n\033[31m✗ peer dependency\033[0m missing" | \
    $BOXY --color blue --title "📦 With Dividers" --layout "dt,ds"

echo -e "\033[32m✓ npm install\033[0m completed\n\033[33m⚠ 3 vulnerabilities\033[0m found\n\033[31m✗ peer dependency\033[0m missing" | \
    $BOXY --color purple --title "📦 All Features" --header "Header" --status "Status" --layout "dt,ds"

echo -e "\033[32m✓ npm install\033[0m completed\n\033[33m⚠ 3 vulnerabilities\033[0m found\n\033[31m✗ peer dependency\033[0m missing" | \
    $BOXY --color yellow

# Problem child: Mixed symbol width alignment test
echo -e "✓ npm install completed\n⚠ 3 vulnerabilities found\n✗ peer dependency missing" | \
    $BOXY --color orange --title "🔍 Alignment Test"

# Test variations to isolate the padding issue
echo -e "✓ npm install completed\n⚠ 3 vulnerabilities found\n✗ peer dependency missing" | \
    $BOXY --color red --title "🔍 With Status" --status "Test status"

echo -e "✓ npm install completed\n⚠ 3 vulnerabilities found\n✗ peer dependency missing" | \
    $BOXY --color green --title "🔍 With Header" --header "Test header"

echo -e "✓ npm install completed\n⚠ 3 vulnerabilities found\n✗ peer dependency missing" | \
    $BOXY --color blue --title "🔍 With Dividers" --layout "dt,ds"

echo -e "✓ npm install completed\n⚠ 3 vulnerabilities found\n✗ peer dependency missing" | \
    $BOXY --color purple --title "🔍 All Features" --header "Header" --status "Status" --layout "dt,ds"

echo -e "✓ npm install completed\n⚠ 3 vulnerabilities found\n✗ peer dependency missing" | \
    $BOXY --color yellow

# 6. EMOJI AND UNICODE SHOWCASE
echo -e "\n🎭 EMOJI & UNICODE SHOWCASE:"
cat << 'EOF' | $BOXY --style rounded \
                    --color cyan \
                    --title "🌍 Global Status" \
                    --width 45 \
                    --text auto
🇺🇸 US East:      🟢 Operational  
🇪🇺 EU Central:   🟢 Operational
🇯🇵 Asia Pacific: 🟡 Degraded
🇧🇷 South America: 🔴 Outage

📊 Traffic: 1.2M requests/hour
⏱️  Avg Response: 145ms
💾 Storage: 67% utilized
🔄 Sync Status: Up to date
EOF

# 7. CONFIGURATION SHOWCASE
echo -e "\n⚙️  CONFIGURATION SHOWCASE:"
export BOXY_THEME=info
export BOXY_MIN_WIDTH=30
echo "Using environment variables for theming" | $BOXY --status "sl:BOXY_THEME=info"

# 8. PARAMETER STREAMS
echo -e "\n🔄 PARAMETER STREAMS:"
echo "Body content from stdin" | $BOXY --params "hd='Stream Processing'; tl='🌊 Data Flow'; ft='Pipeline v2.1'"

# 9. WIDTH AND TRUNCATION
echo -e "\n📏 WIDTH MANAGEMENT:"
echo "This is a very long message that demonstrates boxy's intelligent text truncation and width management capabilities in action" | \
    $BOXY --width 30 --theme info --title "📐 Auto-truncation"

# 10. COMPLEX REAL-WORLD SCENARIO
echo -e "\n🏢 REAL-WORLD SCENARIO:"
cat << 'EOF' | $BOXY --theme blueprint \
                    --title "🎉 Release v2.4.0 Deployed" \
                    --header "Production Deployment Summary" \
                    --footer "Next release: v2.5.0 (Oct 15)" \
                    --status "sc:🎯 Zero downtime deployment ✨" \
                    --width 65 \
                    --pad ab \
                    --layout "dt,ds,stn,ssn"
🚀 DEPLOYMENT COMPLETED SUCCESSFULLY

📋 Release Notes:
   • New user dashboard with real-time analytics
   • Enhanced security with 2FA integration  
   • Performance improvements (40% faster)
   • Bug fixes and stability improvements
   • Mobile app synchronization

📊 Metrics:
   • Deploy time: 4m 23s
   • Tests passed: 1,247/1,247
   • Code coverage: 96%
   • Performance score: A+

🔗 Links:
   • Documentation: docs.company.com/v2.4.0
   • Release notes: github.com/company/app/releases
   • Support: help.company.com

Thank you to all contributors who made this release possible!
EOF

# 11. MULTI-STYLE SHOWCASE
echo -e "\n🎯 MULTI-STYLE SHOWCASE:"
echo "Normal borders" | $BOXY --style normal --color red
echo "Rounded corners" | $BOXY --style rounded --color green  
echo "Double lines" | $BOXY --style double --color blue
echo "Heavy borders" | $BOXY --style heavy --color purple
echo "ASCII compatible" | $BOXY --style ascii --color orange

# 12. HEIGHT CONTROL SHOWCASE
echo -e "\n📏 HEIGHT CONTROL & TERMINAL LAYOUT:"

# Perfect dashboard with fixed height for consistent terminal layout
cat << 'EOF' | $BOXY --theme blueprint \
                    --title "📊 System Monitoring Dashboard" \
                    --header "Real-time Infrastructure Status" \
                    --footer "Last updated: $(date '+%H:%M:%S')" \
                    --status "sc:⚡ All systems operational" \
                    --width 60 \
                    --height 15 \
                    --layout "dt,ds,stn,ssn"
🖥️  Server Load: 34% CPU, 67% Memory
🌐 Network: 145 Mbps in, 89 Mbps out
💾 Storage: 2.4TB free of 8TB total
🔒 Security: All firewalls active
📈 Uptime: 127 days, 14 hours
EOF

# Terminal multiplexer-style consistent height boxes
echo "Production deployment status" | $BOXY --theme success \
                                         --title "🚀 Deploy #142" \
                                         --status "sl:✅ Completed successfully" \
                                         --height 10 \
                                         --width 45

echo "Database backup running..." | $BOXY --theme info \
                                        --title "💾 Backup Service" \
                                        --status "sr:⏳ 67% complete" \
                                        --height 10 \
                                        --width 45

# Height with params - Configuration management style
cat << 'EOF' | $BOXY --params "h=18; w=50; tl='⚙️ Configuration Manager'; hd='Environment: Production'; ft='Config v2.1.0'; st='🎯 All validations passed';"
API Endpoints:
  • Authentication: https://auth.api.company.com
  • User Service: https://users.api.company.com
  • Payment Gateway: https://pay.api.company.com
  • Analytics: https://analytics.api.company.com

Database Connections:
  • Primary DB: postgresql://prod-primary:5432
  • Read Replicas: 3 active connections
  • Redis Cache: redis://cache-cluster:6379
  • Search Index: elasticsearch://search:9200

Security Settings:
  • SSL/TLS: Enabled (TLS 1.3)
  • API Rate Limiting: 1000 req/min
  • CORS Origins: app.company.com
  • JWT Expiry: 24 hours
EOF

# Layout engine demonstration - consistent heights for grid layouts
echo -e "\n🏗️  LAYOUT ENGINE - Consistent Grid Heights:"
echo "Service A" | $BOXY --theme success --height 8 --width 20 --title "📦 App Server" &
echo "Service B" | $BOXY --theme warning --height 8 --width 20 --title "🗄️ Database" &
echo "Service C" | $BOXY --theme info --height 8 --width 20 --title "🔍 Search" &
wait

# Advanced terminal UI with height control
echo -e "\n🎛️  ADVANCED TERMINAL UI:"
cat << 'EOF' | $BOXY --theme magic \
                    --title "✨ Deployment Wizard" \
                    --header "Step 3 of 5: Environment Configuration" \
                    --footer "Press ENTER to continue, ESC to cancel" \
                    --status "sc:🎯 Ready to deploy" \
                    --height 20 \
                    --width 65 \
                    --layout "dt,ds,stn,ssn"
📋 Deployment Configuration Summary:

   Environment: Production
   Region: us-east-1
   Instance Type: t3.large (2 vCPU, 8GB RAM)
   Auto Scaling: 2-10 instances
   Load Balancer: Application Load Balancer
   Database: RDS PostgreSQL 13.7

🔐 Security Configuration:
   ✅ WAF Protection enabled
   ✅ SSL certificate configured
   ✅ VPC security groups applied
   ✅ IAM roles and policies set
   ✅ CloudTrail logging enabled

📊 Monitoring Setup:
   ✅ CloudWatch metrics enabled
   ✅ Application performance monitoring
   ✅ Log aggregation configured
   ✅ Alerting rules established

Ready to proceed with deployment?
EOF

# 13. NO-BOXY PIPELINE DEMO
echo -e "\n🔧 PIPELINE INTEGRATION:"
echo "Raw content for pipeline processing" | $BOXY --theme error --no-boxy
echo "Completely stripped content" | $BOXY --theme success --no-boxy=strict

echo -e "\n========================================="
echo "🎊 PERFECT DEMO COMPLETE!"
echo "🏆 Boxy v0.8 - All features demonstrated"
echo "🎯 HEIGHT MILESTONE: Terminal layout control"
echo "========================================="