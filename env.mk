check_tool = $(shell which $(1) 2>/dev/null)
check_cargo_cmd = $(shell cargo --list | grep "$(1)" 2>/dev/null)

# Output mode: full (default) or quiet (warnings/errors only)
ENV_CHECK_MODE ?= full

# Define tool lists
REQUIRED_TOOLS := cargo rustc
OPTIONAL_TOOLS := qemu-system-$(ARCH) gdb lldb
REQUIRED_CARGO_CMDS := objcopy

define HELP_INSTALL
Required tools are missing. Please install:

1. Rust toolchain:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Cargo tools:
   cargo install cargo-binutils
endef

.PHONY: check-env
check-env: ## Check toolchain and optional tools
	@if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
		echo "➤ Checking environment..."; \
		echo "╔═══════════════════════════════════════╗"; \
		echo "║ Required Tools                        ║"; \
		echo "╚═══════════════════════════════════════╝"; \
	fi
	@EXIT_CODE=0; \
	for tool in $(REQUIRED_TOOLS); do \
		if [ -z "$$(which $$tool 2>/dev/null)" ]; then \
			echo "❌ $$tool: Not found"; \
			[ "$$tool" = "cargo" -o "$$tool" = "rustc" ] && \
				echo "   Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"; \
			EXIT_CODE=1; \
		else \
			if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
				echo "✅ $$tool: $$(which $$tool)"; \
			fi; \
		fi; \
	done; \
	if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
		echo ""; \
		echo "╔═══════════════════════════════════════╗"; \
		echo "║ Optional Tools                        ║"; \
		echo "╚═══════════════════════════════════════╝"; \
	fi; \
	for tool in $(OPTIONAL_TOOLS); do \
		if [ -z "$$(which $$tool 2>/dev/null)" ]; then \
			echo "⚠️  $$tool: Not found (optional)"; \
		else \
			if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
				echo "✅ $$tool: $$(which $$tool)"; \
			fi; \
		fi; \
	done; \
	if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
		echo ""; \
		echo "╔═══════════════════════════════════════╗"; \
		echo "║ Required Cargo Commands               ║"; \
		echo "╚═══════════════════════════════════════╝"; \
	fi; \
	for cmd in $(REQUIRED_CARGO_CMDS); do \
		if [ -z "$$(cargo --list | grep "$$cmd" 2>/dev/null)" ]; then \
			echo "❌ cargo-$$cmd: Not found"; \
			echo "   Install with: cargo install cargo-binutils"; \
			EXIT_CODE=1; \
		else \
			if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
				echo "✅ cargo-$$cmd: Found"; \
			fi; \
		fi; \
	done; \
	if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
		echo ""; \
	fi; \
	if [ $$EXIT_CODE -eq 0 ]; then \
		if [ "$(ENV_CHECK_MODE)" != "quiet" ]; then \
			echo "✅ All required tools are available."; \
		fi; \
	else \
		echo "❌ Some required tools are missing."; \
		exit $$EXIT_CODE; \
	fi
