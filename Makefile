# used
ARCH          ?= riscv64
MODE          ?= release
LOG           ?= 
GRAPHIC       ?= off
SMP           ?= 1
# unused
PCI_PASSTHRU  ?=
INIT          ?=
EXTRA_NIC     ?= off
ACCEL         ?= off
HYPERVISOR    ?= off
UART2         ?= off

export ARCH
export LOG

# target
ifeq ($(ARCH), riscv64)
	TARGET := riscv64gc-unknown-none-elf
else ifeq ($(ARCH), riscv32)
	TARGET := riscv32gc-unknown-none-elf
endif

KERNEL_NAME := neko-kernel
KERNEL_PATH := target/$(TARGET)/$(MODE)
KERNEL_BIN := $(KERNEL_PATH)/$(KERNEL_NAME).bin
KERNEL_ELF := $(KERNEL_PATH)/$(KERNEL_NAME)

# build options
BUILD_OPTION := \
	--target targets/$(TARGET).json \
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

GDB := gdb
QEMU := qemu-system-$(ARCH)
OBJCOPY := rust-objcopy --binary-architecture=$(ARCH)

__kernel:
	@echo Building $(ARCH) kernel
	@cd kernel && cargo build $(BUILD_OPTION)
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

build: __kernel

run: build
	@$(QEMU) $(QEMU_OPTIONS) -kernel $(KERNEL_BIN)

debug: build
	@$(QEMU) $(QEMU_OPTIONS) $(KERNEL_BIN) -S -s &
	@$(GDB) $(KERNEL_ELF) \
		-ex 'target remote localhost:1234'
		-ex 'continue' \

clean:
	@cargo clean
