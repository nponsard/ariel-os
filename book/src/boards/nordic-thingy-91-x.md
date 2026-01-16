# Nordic Thingy:91 X

## References

- [Manufacturer link](https://web.archive.org/web/20250329185651/https://www.nordicsemi.com/Products/Development-hardware/Nordic-Thingy-91-X)

## laze Builders

For more information on laze builders, check out [this page](../build-system.md#laze-builders).
### `nordic-thingy-91-x-nrf9151`

- **Tier:** 2
- **Chip:** [nRF9151](../chips/nrf9151.md)
- **Chip Ariel OS Name:** `nrf9151`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b nordic-thingy-91-x-nrf9151
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported">✅</span>|
|I2C Controller Mode|<span title="supported">✅</span>|
|SPI Main Mode|<span title="supported">✅</span>|
|UART|<span title="needs testing">🚦</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-supporting-the-onboard-nrf7002-chip]|
|Bluetooth Low Energy|<span title="not available on this piece of hardware">–</span>|
|Hardware Random Number Generator|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^only-available-through-the-cryptocell]|
|Persistent Storage|<span title="supported">✅</span>|
### `nordic-thingy-91-x-nrf5340-net`

- **Tier:** 2
- **Chip:** [nRF5340 network core](../chips/nrf5340-net.md)
- **Chip Ariel OS Name:** `nrf5340-net`

To target this laze builder, run the following command in the root of your Ariel OS app:

```bash
laze build -b nordic-thingy-91-x-nrf5340-net
```

#### Support Matrix

|Functionality|Support Status|
|---|:---:|
|Debug Output|<span title="supported">✅</span>|
|Logging|<span title="supported">✅</span>|
|GPIO|<span title="supported with some caveats">☑️</span>[^pins-need-to-be-assigned-to-the-network-core-from-the-application-core]|
|I2C Controller Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|SPI Main Mode|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|UART|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>|
|User USB|<span title="not available on this piece of hardware">–</span>|
|Ethernet over USB|<span title="not available on this piece of hardware">–</span>|
|Wi-Fi|<span title="available in hardware, but not currently supported by Ariel OS">❌</span>[^requires-supporting-the-onboard-nrf7002-chip]|
|Bluetooth Low Energy|<span title="supported">✅</span>|
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


  
[^requires-supporting-the-onboard-nrf7002-chip]: Requires supporting the onboard nRF7002 chip.
[^only-available-through-the-cryptocell]: Only available through the CryptoCell.
  
[^pins-need-to-be-assigned-to-the-network-core-from-the-application-core]: Pins need to be assigned to the network core from the application core.
[^requires-supporting-the-onboard-nrf7002-chip]: Requires supporting the onboard nRF7002 chip.