# used
ARCH          ?= riscv64
MODE          ?= release
LOG           ?= 
# unused
GRAPHIC       ?= off
SMP           ?= 4
PCI_PASSTHRU  ?=
INIT          ?=
EXTRA_NIC     ?= off
ACCEL         ?= off
HYPERVISOR    ?= off
UART2         ?= off

export ARCH
export LOG

ifeq ($(ARCH), riscv64)
	TARGET := riscv64gc-unknown-none-elf
else ifeq ($(ARCH), riscv32)
	TARGET := riscv32gc-unknown-none-elf
endif

QEMU := qemu-system-$(ARCH)
QEMU_OPTIONS := -machine virt -nographic -kernel

KERNEL_NAME := neko-kernel
KERNEL_PATH := target/$(TARGET)/$(MODE)
KERNEL_BIN := $(KERNEL_PATH)/$(KERNEL_NAME).bin
KERNEL_ELF := $(KERNEL_PATH)/$(KERNEL_NAME)

OBJCOPY := rust-objcopy --binary-architecture=$(ARCH)

__kernel:
	@cd kernel && cargo build --$(MODE)
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

build: __kernel

run: build
	@$(QEMU) $(QEMU_OPTIONS) $(KERNEL_BIN)
	
clean:
	@cargo clean
