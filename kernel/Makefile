ARCH=aarch64-elf
CC=$(ARCH)-gcc
LD=$(ARCH)-ld
OBJCPY=$(ARCH)-objcopy
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

libkernel: src/*.rs
	cargo xbuild --target $(shell pwd)/$(ARCH).json $(CARGO_FLAGS)

$(OUTPUT)/kernel.elf: $(OUTPUT)/start.o libkernel
	$(CC) -Wl,--gc-sections -nostdlib -nostdinc -nostartfiles $(OUTPUT)/start.o -T linker.ld -o $(OUTPUT)/kernel.elf -L$(OUTPUT) -l:libkernel.a

$(OUTPUT)/kernel8.img: $(OUTPUT)/kernel.elf
	$(OBJCPY) -O binary $(OUTPUT)/kernel.elf $(OUTPUT)/kernel8.img

dump: default
	$(ARCH)-objdump -D $(OUTPUT)/kernel.elf > dump.txt