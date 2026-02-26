# Seeed Studio XIAO NRF52840 Plus

## References

- [Manufacturer link](https://web.archive.org/web/20260101232815/https://wiki.seeedstudio.com/XIAO_BLE/)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `seeedstudio-xiao-nrf52840-plus`

- **Tier:** 3
- **Chip:** [nRF52840](../chips/nrf52840.md)
- **Chip Ariel OS Name:** `nrf52840`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b seeedstudio-xiao-nrf52840-plus
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|User USB|<span title="needs testing">🚦</span>|
|Ethernet over USB|<span title="needs testing">🚦</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported">✅</span>|

<p>Legend:</p>

<dl>
  <div>
    <dt>✅</dt><dd>supported</dd>
  </div>
  <div>
    <dt>☑️</dt><dd>supported with some caveats</dd>
  </div>
  <div>
    <dt>🚦</dt><dd>needs testing</dd>
  </div>
  <div>
    <dt>❌</dt><dd>available in hardware, but not currently supported by Ariel OS</dd>
  </div>
  <div>
    <dt>–</dt><dd>not available on this piece of hardware</dd>
  </div>
</dl>
<style>
dt, dd {
  display: inline;
}
</style>


  