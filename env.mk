check_tool = $(shell which $(1) 2>/dev/null)
check_cargo_cmd = $(shell cargo --list | grep "$(1)" 2>/dev/null)

# Check for required tools
REQUIRED_TOOLS := cargo rustc qemu-system-$(ARCH)
$(foreach tool,$(REQUIRED_TOOLS),\
    $(if $(call check_tool,$(tool)),,\
        $(error "$(tool) is not installed")))

# Check for required cargo commands
REQUIRED_CARGO_CMDS := objcopy
$(foreach cmd,$(REQUIRED_CARGO_CMDS),\
    $(if $(call check_cargo_cmd,$(cmd)),,\
        $(error "cargo-$(cmd) is not installed. Run: cargo install cargo-binutils")))

define HELP_INSTALL
Required tools are missing. Please install:

1. Rust toolchain:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Cargo tools:
   cargo install cargo-binutils

3. QEMU:
   # For Ubuntu/Debian
   sudo apt install qemu-system
   # For macOS
   brew install qemu

4. Run 'rustup show' to verify toolchain installation
endef

.PHONY: check-env
check-env:
	@echo "Checking environment..."
	@rustup show
	@rustc --version
	@cargo --version
	@$(QEMU) --version
