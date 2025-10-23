## Bluetooth Low Energy

Ariel os provides an abstraction over [TrouBLE](https://github.com/embassy-rs/trouble). The hardware specific initialization is handled by Ariel OS for supported chips.

### Enabling the BLE stack

BLE is enabled by selecting one or both of the BLE [laze modules][laze-modules-book] depending on your use case:

- `ble-peripheral`: for advertising data, acting as a "sever".
- `ble-central`: for scanning and connecting to peripherals, reading their data.

If you want to learn more about BLE concepts, you can read the [TrouBLE documentation](https://embassy.dev/trouble/#_concepts).

### Using the BLE stack

To use BLE in your application, you need to get the pre-configured stack and build the host object from it:

```rust
let stack = ariel_os::ble::ble_stack().await;
let host = stack.build();
```

Here `stack` is of type [`trouble_host::Stack`](https://docs.embassy.dev/trouble-host/0.1.0/default/struct.Stack.html) and `host` is of type [`trouble_host::Host](https://docs.embassy.dev/trouble-host/0.1.0/default/struct.Host.html)

You then need to run the runner background task in your application.

```rust
host.runner.run().await;
```

> Note: the runner has other options like `run_with_handler(&lt;handler&gt;) that you may want to use (when scanning for example).

This function will return a result only when a critical error happened in the BLE stack. For most use cases you will need to execute code alongside the runner in the same task (that accesses elements of the `stack` or `host`), you can do that using `embassy_futures::join::join`:

```rust
embassy_futures::join::join(host.runner.run(), async {
  // Execute BLE-related code here that use `host` or `stack`
}).await;
```

See [examples](https://github.com/ariel-os/ariel-os/tree/main/examples) for more details.

[laze-modules-book]: ./build-system.md#laze-modules
