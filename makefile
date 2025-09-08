SHELL := /bin/bash
.DEFAULT_GOAL := help

# Includes (optional)
-include project.mk
-include conf/makefile-bug.mk
-include conf/makefile-semv.mk
-include conf/makefile-project.mk

.PHONY: lint-sh lint test deploy install help

lint-sh:
	bash bin/lint-sh

# Alias target
lint: lint-sh

# Simple test alias to keep muscle memory
test:
	bash bin/test.sh list

# Deployment wrappers (delegate to existing script)
DEPLOY := bin/deploy.sh

deploy: guard.bug-off
	bash $(DEPLOY)

install: deploy

.PHONY: deploy-tag
ENFORCE_TAG ?= 1
deploy-tag: guard.bug-off
	@test -n "$(TAG)" || { echo 'TAG is required, e.g., make deploy-tag TAG=v1.2.3'; exit 2; }
	@git rev-parse -q --verify "refs/tags/$(TAG)" >/dev/null || { echo "Tag $(TAG) not found"; exit 2; }
	@if [ "$(ENFORCE_TAG)" = "1" ]; then \
	  if ! git describe --tags --exact-match >/dev/null 2>&1; then \
	    echo "Current HEAD is not at tag $(TAG). Checkout the tag or set ENFORCE_TAG=0"; exit 2; \
	  fi; \
	  if [ -z "$(ALLOW_DIRTY)" ] || [ "$(ALLOW_DIRTY)" = "0" ]; then \
	    git diff-index --quiet HEAD -- || { echo "Working tree not clean; set ALLOW_DIRTY=1 to override"; exit 2; }; \
	  fi; \
	  current_tag=$$(git describe --tags --exact-match 2>/dev/null || true); \
	  if [ "$$current_tag" != "$(TAG)" ]; then \
	    echo "At $$current_tag; expected $(TAG)"; exit 2; \
	  fi; \
	fi
	$(MAKE) deploy

help:
	@echo "Targets:"; \
	echo "  lint-sh    - Lint shell scripts (bash -n, shellcheck if present)"; \
	echo "  lint       - Alias for lint-sh"; \
	echo "  test       - List available tests"; \
	echo "  deploy     - Run bin/deploy.sh"; \
	echo "  deploy-tag - Deploy from a specific tag (TAG=vX.Y.Z)"; \
	echo "  install    - Alias for deploy"; \
	if [ -f makefile-semv.mk ]; then echo "  semv.*    - Version operations (see makefile-semv.mk)"; fi; \
	if [ -f makefile-bug.mk ]; then echo "  bug.*     - Bug mode lock (on/off/status)"; fi; \
	if [ -f makefile-project.mk ]; then echo "  project:* - Project-specific targets"; fi
