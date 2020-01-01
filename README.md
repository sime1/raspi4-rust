# Raspberry Pi 4 Bare Metal Rust

This repo contains some code I wrote trying to learn about bare metal
programming the Pi 4.

## Building

To build a `kernel8.img` file, you need

* cargo xbuild
* an `aarch64-elf` cross compilation toolchain (other toolchains may work, you
  can try changing the `ARCH` in the Makefile)

```shell
make PROFILE=release
```

without `PROFILE=release`, the debug version of the file will be generated. The
output will be placed inside the `kernel/target/aarch64-elf/release` directory.