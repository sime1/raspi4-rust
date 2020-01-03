ARCH=aarch64-elf
CFLAGS=-ffreestanding
PROFILE=debug
OUTPUT=./target/$(ARCH)/$(PROFILE)

ifeq ($(PROFILE), release)
	CARGO_FLAGS=--release
else
	CARGO_FLAGS=
endif

.PHONY: before-all clean

default: before-all $(OUTPUT)/kernel8.img

clean:
	cargo clean

$(OUTPUT)/start.o: start.S
	$(CC) $(CFLAGS) -c start.S -o $(OUTPUT)/start.o

before-all:
	mkdir -p $(OUTPUT)

kernel-elf: **/*.rs
	cargo xbuild --target $(shell pwd)/$(ARCH).json $(CARGO_FLAGS)

$(OUTPUT)/kernel8.img: kernel-elf
	rust-objcopy --strip-all -O binary $(OUTPUT)/kernel $(OUTPUT)/kernel8.img

dump: default
	rust-objdump -D $(OUTPUT)/kernel > dump.txt