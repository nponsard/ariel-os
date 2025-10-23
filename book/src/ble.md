## Bluetooth Low Energy

Ariel os provides an abstraction over [TrouBLE](https://github.com/embassy-rs/trouble). The hardware specific initialization is handled by Ariel OS for supported chips.

To use BLE in your application, you need to get the pre-configured stack and build it:

```rust
let stack = ariel_os::ble::ble_stack().await;
let host = stack.build();
```

You then need to mainain the runner task active:

FIXME: find is this can be in a seperate task/spawned
```rust
host.runner.run().await;
```

> Note: the runner has other options like `run_with_handler(&lt;handler&gt;) that you may want to use (when scanning for example).
