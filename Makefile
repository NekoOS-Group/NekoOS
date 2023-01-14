QEMU := qemu-system-riscv64 -machine virt -nographic

KERNEL_NAME := neko-kernel
KERNEL_PATH := target/riscv64gc-unknown-none-elf/release
KERNEL_BIN := $(KERNEL_PATH)/$(KERNEL_NAME).bin
KERNEL_ELF := $(KERNEL_PATH)/$(KERNEL_NAME)

MAKE := make
OBJCOPY := rust-objcopy --binary-architecture=riscv64

__kernel: kernel
	cd kernel && cargo build --release
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

__ulib:
	cd ulib && cargo build --release

build: __kernel __ulib
	

run: build
	@$(QEMU) -kernel $(KERNEL_BIN)
	
clean:
	rm -rf target && mkdir target
