# STEVAL-MKBOXPRO

## References

- [Manufacturer link](https://web.archive.org/web/20250507145935/https://www.st.com/en/evaluation-tools/steval-mkboxpro.html)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `st-steval-mkboxpro`

- **Tier:** 2
- **Chip:** [STM32U585AI](../chips/stm32u585ai.md)
- **Chip Ariel OS Name:** `stm32u585ai`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b st-steval-mkboxpro
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
|User USB|<span title="supported">✅</span>|
|Ethernet over USB|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^usb-does-not-enumerate-https-github-com-embassy-rs-embassy-issues-2376-workaround-https-github-com-ariel-os-ariel-os-pull-1126]|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
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


  
[^usb-does-not-enumerate-https-github-com-embassy-rs-embassy-issues-2376-workaround-https-github-com-ariel-os-ariel-os-pull-1126]: [USB does not enumerate](https://github.com/embassy-rs/embassy/issues/2376), [workaround](https://github.com/ariel-os/ariel-os/pull/1126).
[^removing-items-not-supported]: Removing items not supported.