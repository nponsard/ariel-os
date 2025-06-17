/// Maximum Transmission Unit (MTU) for BLE connections. 27 should work for all BLE versions.
///
/// Since bluetooth 4.2, this can be increased to 251 bytes.
// TODO: add the ability to configure this value.
pub const MTU: usize = 27;

/// Maximum number of concurrent connections to be handled by the BLE stack, minimum 1.
pub const MAX_CONNS: usize = 1;
/// Maximum number of concurrent channels to be handled by the BLE stack (not including GATT), minimum 1.
pub const MAX_CHANNELS: usize = 1;


/// Maximum amount of packets that can be handled by the BLE stack, minimum 1.
pub const MAX_PACKETS: usize = 16;
