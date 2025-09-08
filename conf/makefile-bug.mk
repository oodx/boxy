## makefile-bug.mk — Bug lock module

# Persistent state file (can override in project.mk)
BUG_STATE_FILE ?= .bugmode

.PHONY: bug.on bug.off bug.status guard.bug-off

bug.on: ## Enable bug mode (lock deployment/publish)
	@touch "$(BUG_STATE_FILE)"; echo "🔒 BUG MODE ON ($(BUG_STATE_FILE))"

bug.off: ## Disable bug mode
	@rm -f "$(BUG_STATE_FILE)"; echo "🔓 BUG MODE OFF"

bug.status: ## Show bug mode status
	@if [ -f "$(BUG_STATE_FILE)" ]; then echo "Bug mode: ON ($(BUG_STATE_FILE))"; else echo "Bug mode: OFF"; fi

# Guard: fail fast when bug mode is active
guard.bug-off:
	@if [ -f "$(BUG_STATE_FILE)" ]; then \
	  echo "❌ Blocked: BUG MODE is ON (found $(BUG_STATE_FILE))."; \
	  echo "   Toggle with 'make bug.off' to proceed."; \
	  exit 2; \
	fi
