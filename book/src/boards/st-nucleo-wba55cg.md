# ST NUCLEO-WBA55CG

## References

- [Manufacturer link](https://web.archive.org/web/20240803070523/https://www.st.com/en/evaluation-tools/nucleo-wba55cg.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `st-nucleo-wba55`

- **Tier:** 3
- **Chip:** [STM32WBA55CG](../chips/stm32wba55cg.md)
- **Chip Ariel OS Name:** `stm32wba55cg`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-wba55
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
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^removing-items-not-supported]|

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