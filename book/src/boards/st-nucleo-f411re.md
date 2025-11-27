# ST NUCLEO-F411RE

## References

- [Manufacturer link](https://web.archive.org/web/20250311221905/https://www.st.com/en/evaluation-tools/nucleo-f411re.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `st-nucleo-f411re`

- **Tier:** 3
- **Chip:** [STM32F411RE](../chips/stm32f411re.md)
- **Chip Ariel OS Name:** `stm32f411re`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-f411re
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="needs testing">🚦</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^unsupported-heterogeneous-flash-organization]|

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


  
[^unsupported-heterogeneous-flash-organization]: Unsupported heterogeneous flash organization.