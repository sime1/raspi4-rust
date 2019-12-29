ARCH=aarch64-elf
CC=$(ARCH)-gcc
LD=$(ARCH)-ld
OBJCPY=$(ARCH)-objcopy
CFLAGS=-ffreestanding
OUTPUT=./target
PROFILE=debug

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
	mkdir -p target

libkernel: src/*.rs
	xargo build --target $(ARCH) $(CARGO_FLAGS)

$(OUTPUT)/kernel.elf: $(OUTPUT)/start.o libkernel
	$(LD) -nostdlib -nostartfiles $(OUTPUT)/start.o -T linker.ld -o $(OUTPUT)/kernel.elf -L$(OUTPUT)/$(ARCH)/$(PROFILE)/ -l:libkernel.a

$(OUTPUT)/kernel8.img: $(OUTPUT)/kernel.elf
	$(OBJCPY) -O binary $(OUTPUT)/kernel.elf $(OUTPUT)/kernel8.img