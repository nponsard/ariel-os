# nRF5340-DK

## References

- [Manufacturer link](https://web.archive.org/web/20250115224621/https://www.nordicsemi.com/Products/Development-hardware/nrf5340-dk)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `nrf5340dk`

- **Tier:** 1
- **Chip:** [nRF5340 application core](../chips/nrf5340.md)
- **Chip Ariel OS Name:** `nrf5340`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b nrf5340dk
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported">✅</span>|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="supported">✅</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="supported">✅</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]|
|Persistent Storage|<span title="supported">✅</span>|
### `nrf5340dk-net`

- **Tier:** 1
- **Chip:** [nRF5340 network core](../chips/nrf5340-net.md)
- **Chip Ariel OS Name:** `nrf5340-net`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b nrf5340dk-net
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|GPIO|<span title="supported with some caveats">☑️</span>[^pins-need-to-be-assigned-to-the-network-core-from-the-application-core]|
|Debug Output|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|Logging|<span title="supported">✅</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="not available on this piece of hardware">–</span>|
|Bluetooth Low Energy|<span title="supported">✅</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="supported">✅</span>|
|Persistent Storage|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|

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


  
[^no-standalone-rng-in-the-application-core-only-in-the-cryptocell-which-is-not-currently-supported]: No standalone RNG in the application core, only in the CryptoCell which is not currently supported.
  
[^pins-need-to-be-assigned-to-the-network-core-from-the-application-core]: Pins need to be assigned to the network core from the application core.