# project.mk — repository-specific configuration for generic makefile

# Binary/install locations (override as needed)
BINARY_NAME := boxy
INSTALL_DIR := $(HOME)/.local/bin/odx

# semv integration knobs (overrides)
SEMV_MIN_SUPPORT := 1.0.0
SEMV_COMPAT := min
SEMV_AUTO_INIT := 0
SEMV_BASH_FILES := bin/deploy.sh
