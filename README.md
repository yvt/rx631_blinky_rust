# Rust Blinky on Renesas [RX][][631][]

[RX]: https://en.wikipedia.org/w/index.php?title=RX_microcontroller_family
[631]: https://www.renesas.com/us/en/products/microcontrollers-microprocessors/rx-32-bit-performance-efficiency-mcus/rx631-32-bit-microcontrollers-enhanced-security-image-capture

## Prerequisites

- rustup
- [`rustc_codegen_gcc`](https://github.com/rust-lang/rustc_codegen_gcc) (experimental GCC codegen for rustc)
    - binutils configured with `--target=rx-elf`
    - [A patched version of libgccjit](https://github.com/rust-lang/rustc_codegen_gcc/blob/14c33f592ae9ecd65c5f7f2436350e8489972a60/Readme.md#building), configured with `--target rx-elf --enable-languages=c,jit --enable-host-shared --disable-werror --disable-multilib --disable-libssp`
    - [A custom fork](https://github.com/yvt/rustc_codegen_gcc/tree/rx/v2) of `rustc_codegen_gcc` for `rx-none-elf` target, built by `./build.sh --release-sysroot`
- [GR-CITRUS](https://www.renesas.com/us/en/products/gadget-renesas/boards/gr-citrus)

## Building and Flashing

Enter the USB mass storage loader mode by pressing the reset button on your board while it's connected to a host machine.

```shell
rustc_codegen_gcc/cargo.sh build --release
rx-elf-objcopy -O binary target/rx-none-elf/release/rx-rusty-blinky /tmp/app.bin
cp /tmp/app.bin /run/media/$USER/GR-CITRUS_b
```

