# Raspberry Pi 4 Bare Metal Rust

This repo contains some code I wrote trying to learn about bare metal
programming the Pi 4.

## Dependencies

To build a `kernel8.img` file, you need an `aarch64-elf` cross compilation 
toolchain (other toolchains may work, you can try changing the `ARCH` in the 
Makefile).

You also need to have a working rust installation. Then to install the needed
dependecies run 

```shell
rustup component add rust-src
cargo install cargo-xbuild
rustup override set nightly
```
(The las command needs to be run inside the directory of the repo)

## Building

```shell
make PROFILE=release
```

without `PROFILE=release`, the debug version of the file will be generated. The
output will be placed inside the `target/aarch64-elf/release` directory.

