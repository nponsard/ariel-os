//! Common BLE types to be used across different HALs.

use static_cell::StaticCell;
use trouble_host::HostResources;

mod config;
mod packet_pool;

pub use config::{MAX_CHANNELS, MAX_CONNS, MTU};
pub use packet_pool::BlePacketPool;

static HOST_RESOURCES: StaticCell<BleHostResource> = StaticCell::new();

/// Alias for the BLE host resources, setting the generic parameters to the constants for convenience.
pub type BleHostResource = HostResources<packet_pool::BlePacketPool, MAX_CONNS, MAX_CHANNELS>;

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
