//! Common BLE types to be used across different HALs.

use static_cell::StaticCell;
use trouble_host::HostResources;

/// Maximum Transmission Unit (MTU) for BLE connections. 27 should work for all BLE versions.
///
/// Since bluetooth 4.2, this can be increased to 251 bytes.
// TODO: add the ability to configure this value.
pub const MTU: usize = 27;

/// Maximum number of concurrent connections to be handled by the BLE stack, minimum 1.
pub const MAX_CONNS: usize = 1;
/// Maximum number of concurrent channels to be handled by the BLE stack (not including GATT), minimum 1.
pub const MAX_CHANNELS: usize = 1;

static HOST_RESOURCES: StaticCell<BleHostResource> = StaticCell::new();

/// Alias for the BLE host resources, setting the generic parameters to the constants for convenience.
pub type BleHostResource = HostResources<MAX_CONNS, MAX_CHANNELS, MTU>;

/// Initialize the BLE host resources.
///
/// Call this function to get the `HostResources` instance that can be used to initialeze the trouBLE stack.
pub fn get_ble_host_resources() -> &'static mut BleHostResource {
    HOST_RESOURCES.init(HostResources::new())
}

/// Configuration for the BLE stack.
///
/// You can customize it using the `ble-config-override` feature.
pub struct Config {
    /// The address of the BLE device.
    pub address: trouble_host::Address,
}

/// Default address, this one is used in the trouBLE examples, needs to be changed/randomized in production.
impl Default for Config {
    fn default() -> Self {
        Self {
            address: trouble_host::Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]),
        }
    }
}
