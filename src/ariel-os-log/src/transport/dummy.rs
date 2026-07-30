/// Register custom transport functions.
pub fn register_transport_fns(_write_bytes_fn: fn(&[u8]), _flush_fn: fn()) {}

// Write bytes to logging transport.
// Must not fail.
pub(crate) fn write_bytes(_bytes: &[u8]) {}
// Flush logging transport.
// Must not fail.
#[cfg(feature = "defmt")]
pub(crate) fn flush() {}
