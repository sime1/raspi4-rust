# Raspberry Pi 4 Bare Metal Rust

This repo contains some code I wrote trying to learn about bare metal
programming the Pi 4.

## Dependencies

* A working rust installation
* cargo xbuild
* cargo binutils

On linux, you can use the following commands (run one command at a time):

```shell
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# install cargo-xbuild
rustup component add rust-src
cargo install cargo-xbuild
# install cargo-binutils
cargo install cargo-binutils
rustup component add llvm-tools-preview
# use the nightly toolchain
rustup override set nightly
```
(The last command needs to be run inside the directory of the repo)

## Building

```shell
make PROFILE=release
```

without `PROFILE=release`, the debug version of the file will be generated. The
output will be placed inside the `target/aarch64-elf/release` directory.

