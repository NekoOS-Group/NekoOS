# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
NAME := libulib.rlib
ULIB := target/$(TARGET)/$(MODE)/$(NAME)

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
