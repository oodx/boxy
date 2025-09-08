# makefile-project.mk — optional repo-specific targets
# Set availability flag so help can reflect presence
PROJ_UNAVAIL := 0

.PHONY: project.info

project.info:
	@echo "Project targets active for repo: $(notdir $(CURDIR))"
