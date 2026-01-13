# ST NUCLEO-C031C6

## References

- [Manufacturer link](https://web.archive.org/web/20241114214921/https://www.st.com/en/evaluation-tools/nucleo-c031c6.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `st-nucleo-c031c6`

- **Tier:** 1
- **Chip:** [STM32C031C6](../chips/stm32c031c6.md)
- **Chip Ariel OS Name:** `stm32c031c6`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-nucleo-c031c6
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
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="not available on this piece of hardware">–</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^would-need-to-allocate-some-flash]|

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


  
[^would-need-to-allocate-some-flash]: Would need to allocate some flash.