# Seeed Studio LoRa-E5 mini

## References

- [Manufacturer link](https://web.archive.org/web/20250802201959/https://wiki.seeedstudio.com/LoRa_E5_mini/)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `seeedstudio-lora-e5-mini`

- **Tier:** 3
- **Chip:** [STM32WLE5JC](../chips/stm32wle5jc.md)
- **Chip Ariel OS Name:** `stm32wle5jc`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b seeedstudio-lora-e5-mini
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="supported with some caveats">☑️</span>[^removing-items-not-supported]|

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


  
[^removing-items-not-supported]: Removing items not supported.