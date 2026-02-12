.DEFAULT_GOAL := help

ARCH          ?= riscv64
MODE          ?= debug
LOG           ?= INFO
GRAPHIC       ?= off
SMP           ?= 1
V             ?= 1

ifeq ($(V),0)
Q := @
else
Q :=
endif

include env.mk

export ARCH
export LOG

TARGET_riscv64 := riscv64gc-unknown-none-elf
TARGET_riscv32 := riscv32gc-unknown-none-elf
GDB_ARCH_riscv64 := riscv:rv64
GDB_ARCH_riscv32 := riscv:rv32

TARGET       := $(TARGET_$(ARCH))
GDB_ARCH     := $(GDB_ARCH_$(ARCH))

KERNEL_PATH  := target/$(TARGET)/$(MODE)
KERNEL_ELF   := $(KERNEL_PATH)/kernel
KERNEL_BIN   := $(KERNEL_PATH)/neko-kernel.bin

# build options
CARGO_BUILD_FLAGS := --target $(TARGET)
ifeq ($(MODE), release)
	CARGO_BUILD_FLAGS += --release
endif

# qemu options
QEMU_OPTIONS := -machine virt -serial mon:stdio -smp cores=$(SMP)

ifeq ($(GRAPHIC), off)
	QEMU_OPTIONS += -nographic
endif

GDB := gdb-multiarch
LLDB := lldb
QEMU := qemu-system-$(ARCH)

.PHONY: help
help: ## Display this help message
	@echo "NekoOS Makefile Help"
	@echo "===================="
	@echo
	@echo "Configuration:"
	@printf "  %-18s %s\n" "ARCH=$(ARCH)" "# Target architecture (riscv64, riscv32)"
	@printf "  %-18s %s\n" "MODE=$(MODE)" "# Build mode (debug, release)"
	@printf "  %-18s %s\n" "LOG=$(LOG)" "# Compile-time log level"
	@printf "  %-18s %s\n" "GRAPHIC=$(GRAPHIC)" "# Enable/disable graphics (on, off)"
	@printf "  %-18s %s\n" "SMP=$(SMP)" "# Number of cores"
	@printf "  %-18s %s\n" "V=$(V)" "# Verbosity (1=show commands, 0=quiet)"
	@echo
	@echo "Available targets:"
	@awk -F ':.*## ' '\
		/^[a-zA-Z0-9_.-]+:.*## / { \
			if (length($$1) > max) max = length($$1); \
			lines[NR] = $$0; \
		} \
		END { \
			for (i = 1; i <= NR; i++) { \
				if (lines[i] ~ /:.*## /) { \
					split(lines[i], a, ":.*## "); \
					printf "  \033[36m%-*s\033[0m %s\n", max, a[1], a[2]; \
				} \
			} \
		}' $(MAKEFILE_LIST)
	@echo
	@echo "Example usage:"
	@echo "  make build ARCH=riscv64 MODE=debug    # Build kernel in debug mode"
	@echo "  make run                              # Run kernel in QEMU"

__nm: ## List symbols in the kernel binary (sorted by size)
	$(Q)cd kernel && cargo nm $(CARGO_BUILD_FLAGS) -- --print-size --size-sort

__kernel: ## Build kernel binary
	@echo Building $(ARCH) kernel
	$(Q)cd kernel && cargo objcopy $(CARGO_BUILD_FLAGS) -- \
        --binary-architecture=$(ARCH) \
	    --strip-all -O binary ../$(KERNEL_BIN)

build: check-env __kernel ## Build the kernel
build: ENV_CHECK_MODE=quiet

run: build ## Run the kernel in QEMU
	$(Q)$(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN)

debug: build ## Start QEMU in debug mode and wait for debugger connection
	@echo "Debug Begin"
	$(Q)$(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN) \
		-S -gdb tcp::1234

debug-screen: build ## Start QEMU in debug mode in a screen session
	@echo "Debug Begin"
	$(Q)screen -dm $(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN) \
		-S -gdb tcp::1234

gdb: debug-screen ## Debug using GDB
	$(Q)$(GDB) ${KERNEL_ELF} \
		-ex 'set architecture $(GDB_ARCH)' \
		-ex 'target remote localhost:1234' \
		-ex 'b start' \
		-ex 'c' \
		-ex 'layout split'

lldb: debug-screen ## Debug using LLDB
	$(Q)$(LLDB) ${KERNEL_ELF}\
		-o 'gdb-remote localhost:1234' \
		-o 'b start'

clean: ## Clean build artifacts
	$(Q)cargo clean
