# Remove the default goal and let help be the first target
# used
ARCH          ?= riscv64
MODE          ?= debug
LOG           ?= INFO
GRAPHIC       ?= off
SMP           ?= 1
# unused
PCI_PASSTHRU  ?=
INIT          ?=
EXTRA_NIC     ?= off
ACCEL         ?= off
HYPERVISOR    ?= off
UART2         ?= off

include env.mk

export ARCH
export LOG

# target
ifeq ($(ARCH), riscv64)
	TARGET    := riscv64gc-unknown-none-elf
	GDB_ARCH  := riscv:rv64
else ifeq ($(ARCH), riscv32)
	TARGET    := riscv32gc-unknown-none-elf
	GDB_ARCH  := riscv:rv64
endif

KERNEL_NAME   := neko-kernel
KERNEL_PATH   := target/$(TARGET)/$(MODE)
KERNEL_BIN    := $(KERNEL_PATH)/$(KERNEL_NAME).bin
KERNEL_ELF    := $(KERNEL_PATH)/$(KERNEL_NAME)

# build options
BUILD_OPTION := \
	--target $(TARGET) \
	-Z build-std=core,alloc

ifeq ($(MODE), release) 
	BUILD_OPTION += --release
endif

# qemu options
QEMU_OPTIONS := -smp cores=$(SMP)

ifeq ($(ARCH), riscv64)
	QEMU_OPTIONS += \
		-machine virt \
		-serial mon:stdio
else ifeq ($(ARCH), riscv32)
	QEMU_OPTIONS += \
		-machine virt \
		-serial mon:stdio
endif

ifeq ($(GRAPHIC), off)
	QEMU_OPTIONS += -nographic
endif

GDB := gdb-multiarch
LLDB := lldb
QEMU := qemu-system-$(ARCH)
OBJCOPY := cargo objcopy ---binary-architecture=$(ARCH)

.PHONY: help
help: ## Display this help message
	@echo "NekoOS Makefile Help"
	@echo "===================="
	@echo
	@echo "Configuration:"
	@echo "  ARCH=$(ARCH)         # Target architecture (riscv64, riscv32)"
	@echo "  MODE=$(MODE)         # Build mode (debug, release)"
	@echo "  LOG=$(LOG)           # Log level"
	@echo "  GRAPHIC=$(GRAPHIC)   # Enable/disable graphics (on, off)"
	@echo "  SMP=$(SMP)           # Number of cores"
	@echo
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'
	@echo
	@echo "Example usage:"
	@echo "  make build ARCH=riscv64 MODE=debug    # Build kernel in debug mode"
	@echo "  make run                              # Run kernel in QEMU"

__nm: ## List symbols in the kernel binary (sorted by size)
	@cd kernel && cargo nm $(BUILD_OPTION) -- --print-size --size-sort

__kernel: ## Build kernel binary
	@echo Building $(ARCH) kernel
	@cd kernel && cargo objcopy $(BUILD_OPTION) -- \
        --binary-architecture=$(ARCH) \
	    --strip-all -O binary ../$(KERNEL_BIN)

build: check-env __kernel ## Build the kernel

run: build ## Run the kernel in QEMU
	@$(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN)

debug: build ## Start QEMU in debug mode and wait for debugger connection
	@echo "Debug Begin"
	@$(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN) \
		-S -gdb tcp::1234

debug-screen: build ## Start QEMU in debug mode in a screen session
	@echo "Debug Begin"
	@screen -dm $(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN) \
		-S -gdb tcp::1234

gdb: debug-screen ## Debug using GDB
	@$(GDB) ${KERNEL_ELF} \
		-ex 'target remote localhost:1234' \
		-ex 'b start' \
		-ex 'c' \
		-ex 'layout split'

lldb: debug-screen ## Debug using LLDB
	@$(LLDB) ${KERNEL_ELF}\
		-o 'gdb-remote localhost:1234' \
		-o 'b start'

clean: ## Clean build artifacts
	@cargo clean
