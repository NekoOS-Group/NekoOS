# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
NAME := neko-kernel
KERNEL_ELF := target/$(TARGET)/$(MODE)/$(NAME)
KERNEL_BIN := $(KERNEL_ELF).bin

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)
	
build: env $(KERNEL_BIN)
	cp $(KERNEL_BIN) ../target/$(KERNEL_BIN_NAME)

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils

kernel:
	cargo build --release

clean:
	cargo clean
