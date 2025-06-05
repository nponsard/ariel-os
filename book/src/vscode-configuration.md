# VSCode configuration

This chapter covers how to setup [Visual Studio Code](https://code.visualstudio.com/) to get features in-editor linting, go to definition, documentation on hover, inlay hints.

## Extensions

Rust language support is provided by `rust-analyzer`, available on the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [Open VSX Registry](https://open-vsx.org/extension/rust-lang/rust-analyzer) for open source forks of VSCode.

It is also recommended to use the the `Even Better TOML` exentsion to have TOML support when editing `Cargo.toml` files: [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml), [Open VSX Registry](https://open-vsx.org/extension/tamasfe/even-better-toml).

`Dependi` can be used to view information about crates and their available versions: [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi), [Open VSX Registry](https://open-vsx.org/extension/fill-labs/dependi).

## Configuration for developing Ariel OS apps

This is meant to be used on projects created using the `cargo-generate` command in the getting-started guide. The configuration works by targetting one board, avoiding the linter to be confused about double declarations.

You will need to have a nightly version of Rust installed, you can install the latest one using:

```sh
rustup toolchain install nightly
```

Then install configure the toolchain by running this at the root of your project:

```sh
laze build -D CARGO_TOOLCHAIN=+nightly install-toolchain
```

To generate/update your vscode configuration in `.vscode/settings.json`, run in the root of your project:

```sh
laze build -b <board> vscode-config
```

With `<board>` being the laze identifier of a board your application will run on (e.g. `nrf52840kd`).

If you get an error about the JSON file being malformated, you may have comments or trailing commas in your configuration, try removing them in `.vscode/settings.json`.

### Debugging

<!-- TODO -->

## Configuration for the Ariel OS repository

<!-- TODO: reword & explain config -->

This sets a default mcu for each manufacturer (stm, nrf, rp, esp).

`.vscode/settings.json`

```json
{
  "rust-analyzer.cargo.features": [
    "coap",
    "csprng",
    "dns",
    "external-interrupts",
    "hwrng",
    "i2c",
    "mdns",
    "net",
    "spi",
    "storage",
    "tcp",
    "udp",
    "usb",
    "usb-ethernet",
    "semihosting",
    "single-core",
    "executor-interrupt",
    "rtt-target",
    "panic-printing",
    "defmt",
    "debug-console",
    // "wifi-esp",
    "wifi-cyw43"
  ],
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.server.extraEnv": {
    "RUSTUP_TOOLCHAIN": "nightly",
    "CARGO_BUILD_TARGET": "thumbv7em-none-eabi",
    "CARGO_TARGET_THUMBV7EM_NONE_EABI_RUSTFLAGS": "--cfg context=\"stm32wb55rg\" --cfg context=\"stm32\" --cfg context=\"rp\" --cfg context=\"rp235xa\" --cfg context=\"rpi-pico2-w\" --cfg context=\"nrf52\" --cfg context=\"nrf52840\" --cfg context=\"nrf\" --cfg context=\"espressif-esp32-c6-devkitc-1\" --cfg context=\"esp32-c6-wroom-1\" --cfg context=\"esp32c6\" --cfg context=\"esp\" --cfg context=\"ariel-os\" --cfg context=\"default\" -Clink-arg=-Tlinkall.x -C force-frame-pointers --cfg stable -Cembed-bitcode=yes -Clto=fat -Ccodegen-units=1 -Clink-arg=-Tdefmt.x --cfg context=\"riscv\" -Clink-arg=-Tisr_stack.x -Clink-arg=-Tlinkme-region-alias.x -Clink-arg=-Tlinkme.x"
  },
  "rust-analyzer.cargo.allTargets": false
}
```
