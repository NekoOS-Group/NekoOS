QEMU := qemu-system-riscv64 -machine virt -nographic

KERNEL_NAME := neko-kernel.bin
KERNEL_PATH := target/${KERNEL_NAME}
KERNEL_ENTRY_PA := 0x80200000

build:
	make build KERNEL_BIN_NAME=${KERNEL_NAME} -C kernel

run: build
	@$(QEMU) -kernel ${KERNEL_PATH}