# Debug Logging

Ariel OS supports debug logging on all platforms. It is enabled by default.

## Logging

Within Rust code, import `ariel_os::debug::log` items, then use Ariel OS logging macros:

```rust
use ariel_os::debug::log::info;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello!");
}
```

## Filtering Logs

In Ariel OS, the log level defaults to `info`. It can be configured using the
laze variable `LOG`.
Depending on the logger, it may be possible to configure different levels per crate or per module.

Example:

```shell
$ laze build -C examples/log --builders nrf52840dk -DLOG=info run
```

## Logging Facades and Loggers

Ariel OS supports multiple logging facades and loggers.
Only one of them may be enabled at a time;
if none of them are enabled, logging statements become no-operations.
Enabling either the `defmt` or `log` [laze modules][laze-modules-book] allows selecting which logging facade and logger is used.
defmt should be preferred when possible as it results in smaller binaries.

### [defmt]

See the [defmt documentation] for general info on the defmt's facade and logger.

The defmt logger supports configuring the log level per crate and per module, as follows:

```shell
$ laze build -C examples/log --builders nrf52840dk -DLOG=info,ariel_os_rt=trace run
```

Note: On Cortex-M devices, the order of `ariel_os::debug::println!()` output and
      `defmt` log output is not deterministic.

### [log]

Ariel OS's logger for `log` supports configuring the log level globally, but does not currently support per-crate filtering.

[defmt]: https://github.com/knurling-rs/defmt
[defmt documentation]: https://defmt.ferrous-systems.com/
[log]: https://github.com/rust-lang/log
[laze-modules-book]: ./build_system.md#laze-modules
