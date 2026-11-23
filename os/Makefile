TARGET := riscv64gc-unknown-none-elf
MODE := release

KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

BOOTLOADER := ../bootloader/opensbi.bin
KERNEL_ENTRY_PA := 0x80200000

OBJCOPY := rust-objcopy --binary-architecture=riscv64
OBJDUMP := rust-objdump --arch-name=riscv64

.PHONY: build kernel run clean disasm

build: kernel
	$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

kernel:
	cargo build --release

run: build
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

clean:
	cargo clean

disasm: kernel
	$(OBJDUMP) -d $(KERNEL_ELF)
