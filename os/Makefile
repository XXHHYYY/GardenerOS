TARGET := riscv64gc-unknown-none-elf
MODE := release

KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

BOOTLOADER := ../bootloader/opensbi.bin
KERNEL_ENTRY_PA := 0x80200000

OBJCOPY := rust-objcopy --binary-architecture=riscv64
OBJDUMP := rust-objdump --arch-name=riscv64

.DEFAULT_GOAL := build

.PHONY: build user kernel run clean disasm

user:
	$(MAKE) -C ../user build

kernel: user
	cargo build --release

build: kernel
	$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

run: build
	qemu-system-riscv64 \
		-machine virt \
		-smp 1 \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

clean:
	cargo clean
	$(MAKE) -C ../user clean

disasm: kernel
	$(OBJDUMP) -d $(KERNEL_ELF)
