TARGET := riscv64gc-unknown-none-elf
TARGET_DIR := target/$(TARGET)/release
APP_DIR := src/bin

APPS := $(wildcard $(APP_DIR)/*.rs)
BINS := $(patsubst $(APP_DIR)/%.rs,$(TARGET_DIR)/%.bin,$(APPS))

OBJCOPY := rust-objcopy --binary-architecture=riscv64

.DEFAULT_GOAL := build

.PHONY: build elf clean

build: elf
	@set -e; for app in $(APPS); do \
		name=$$(basename "$$app" .rs); \
		$(OBJCOPY) "$(TARGET_DIR)/$$name" --strip-all -O binary "$(TARGET_DIR)/$$name.bin"; \
	done

elf:
	cargo build --release --bins

clean:
	cargo clean
