QEMU := qemu-system-riscv64 -machine virt -nographic
KERNEL := neko-kernel.bin

build:
	make build KERNEL_BIN_NAME=$(KERNEL) -C kernel

run: build
	@$(QEMU) target/$(KERNEL)