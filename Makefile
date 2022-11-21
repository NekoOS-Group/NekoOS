QEMU := qemu-system-riscv64 -machine virt -nographic

KERNEL_NAME := neko-kernel.bin
KERNEL_PATH := target/${KERNEL_NAME}
MAKE := make

.PHONY : all
all: build run

.PHONY : kernel
kernel:
	${MAKE} build KERNEL_BIN_NAME=${KERNEL_NAME} -C kernel

.PHONY : ulib
ulib:
    #make build -C ulib

build:kernel ulib
	
run: build
	@$(QEMU) -kernel ${KERNEL_PATH}
	
clear:
    rm -rf ./target/*
